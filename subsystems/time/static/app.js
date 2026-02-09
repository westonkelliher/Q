let autoTimer = null;

const el = {
  day: document.getElementById('day-val'),
  minute: document.getElementById('minute-val'),
  hhmm: document.getElementById('hhmm-val'),
  lightzone: document.getElementById('lightzone-val'),
  message: document.getElementById('message'),
  hand15: document.getElementById('hand-15'),
  dial45: document.getElementById('dial-45'),
  actionsRow: document.getElementById('actions-row'),
  autoToggle: document.getElementById('auto-toggle'),
  tickSpeedMs: document.getElementById('tick-speed-ms'),
  commandForm: document.getElementById('command-form'),
  commandInput: document.getElementById('command-input'),
};

function setLightzoneStyle(zone) {
  const palette = {
    MORNING: { bg: 'var(--morning)', fg: '#143040' },
    AFTERNOON: { bg: 'var(--afternoon)', fg: '#4a2b00' },
    NIGHT: { bg: 'var(--night)', fg: '#e8defc' },
  };
  const entry = palette[zone] || palette.NIGHT;
  el.lightzone.style.background = entry.bg;
  el.lightzone.style.color = entry.fg;
}

function applyState(data) {
  el.day.textContent = String(data.timestamp.day);
  el.minute.textContent = String(data.timestamp.minute);
  el.hhmm.textContent = data.timestamp.hhmm;
  el.lightzone.textContent = data.timestamp.lightzone;
  setLightzoneStyle(data.timestamp.lightzone);
  update45DialSegments(data.timestamp.minute);
  el.hand15.style.transform = `translateX(-50%) rotate(${data.clock_angles.hand_15_deg}deg)`;
  el.dial45.style.transform = `rotate(${data.clock_angles.dial_45_deg_visual - 90}deg)`;
  el.message.textContent = `${data.message} (advanced: ${data.minutes_advanced}m, total45: ${data.clock_angles.dial_45_deg_total.toFixed(2)}°)`;
}

function update45DialSegments(minuteOfDay) {
  const colors = {
    morning: '#8ed0ff',
    afternoon: '#ffbe55',
    night: '#574478',
  };

  // Segment order starts just to the right of the seam and moves clockwise.
  // Right side = future, left side = past.
  // Each section is 60 minutes wide, so sample at section centers.
  const segmentCenterMinuteOffsets = [30, 90, 150, 210, -210, -150, -90, -30];

  for (let i = 0; i < segmentCenterMinuteOffsets.length; i += 1) {
    const sampleMinute = mod1440(minuteOfDay + segmentCenterMinuteOffsets[i]);
    const zone = lightzoneForMinute(sampleMinute);
    el.dial45.style.setProperty(`--seg${i}`, colors[zone]);
  }
}

function lightzoneForMinute(minuteOfDay) {
  if (minuteOfDay >= 240 && minuteOfDay < 720) return 'morning';
  if (minuteOfDay >= 720 && minuteOfDay < 1200) return 'afternoon';
  return 'night';
}

function mod1440(value) {
  return ((value % 1440) + 1440) % 1440;
}

function getTickSpeedMs() {
  const parsed = Number(el.tickSpeedMs.value);
  if (Number.isNaN(parsed) || parsed <= 0) {
    el.tickSpeedMs.value = '1000';
    return 1000;
  }
  const rounded = Math.round(parsed);
  el.tickSpeedMs.value = String(rounded);
  return rounded;
}

function autoToggleLabel() {
  if (!autoTimer) {
    return 'Auto Tick: Off';
  }
  return `Auto Tick: On (${getTickSpeedMs()}ms)`;
}

function startAutoTick() {
  const speed = getTickSpeedMs();
  autoTimer = setInterval(() => {
    postTick(1);
  }, speed);
  el.autoToggle.textContent = autoToggleLabel();
}

function stopAutoTick() {
  if (!autoTimer) return;
  clearInterval(autoTimer);
  autoTimer = null;
  el.autoToggle.textContent = autoToggleLabel();
}

async function getState() {
  const res = await fetch('/api/state');
  const data = await res.json();
  applyState(data);
}

async function postTick(minutes) {
  const res = await fetch('/api/tick', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ minutes }),
  });
  const data = await res.json();
  applyState(data);
}

async function postCommand(command) {
  const res = await fetch('/api/command', {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify({ command }),
  });
  const data = await res.json();
  applyState(data);
}

async function loadActions() {
  const res = await fetch('/api/actions');
  const actions = await res.json();
  el.actionsRow.innerHTML = '';
  for (const action of actions) {
    const btn = document.createElement('button');
    btn.textContent = `${action.command} (+${action.minutes}m)`;
    btn.addEventListener('click', () => postCommand(action.command));
    el.actionsRow.appendChild(btn);
  }
}

for (const button of document.querySelectorAll('button[data-min]')) {
  button.addEventListener('click', () => {
    const minutes = Number(button.getAttribute('data-min'));
    postTick(minutes);
  });
}

el.autoToggle.addEventListener('click', () => {
  if (autoTimer) {
    stopAutoTick();
    return;
  }

  startAutoTick();
});

const onTickSpeedChange = () => {
  if (autoTimer) {
    stopAutoTick();
    startAutoTick();
  } else {
    el.autoToggle.textContent = autoToggleLabel();
  }
};

el.tickSpeedMs.addEventListener('change', onTickSpeedChange);
el.tickSpeedMs.addEventListener('input', onTickSpeedChange);

el.commandForm.addEventListener('submit', (event) => {
  event.preventDefault();
  const command = el.commandInput.value.trim();
  if (!command) return;
  el.commandInput.value = '';
  postCommand(command);
});

loadActions().then(getState);
el.autoToggle.textContent = autoToggleLabel();
