// ===========================================
// Graphic System - Defines visual representations
// ===========================================

/**
 * Creates a Graphic definition object.
 * @param {string[]|object} definition - Either:
 *   - Array of RPG Awesome icon class names (without 'ra-' prefix), OR
 *   - Shape object: { shape: 'line'|'circle'|'rect', dimensions: {...}, strokeCap?: 'round'|'square'|'butt' }
 * @param {string} foreground - CSS color for the main fill
 * @param {string} background - CSS color for the outline/stroke
 * @param {object} options - Optional settings: strokeWidth, scaleX, scaleY, rotation (degrees)
 */
function createGraphic(definition, foreground, background, options = {}) {
    return {
        type: Array.isArray(definition) ? 'icon' : 'shape',
        definition,  // either icons array or shape object
        foreground,
        background,
        strokeWidth: options.strokeWidth ?? 3,
        scaleX: options.scaleX ?? 1,
        scaleY: options.scaleY ?? 1,
        rotation: options.rotation ?? 0,  // degrees
    };
}

// Game Graphics Definitions
const GRAPHICS = {
    // Character variants
    player: createGraphic(['player'], '#e4b574', '#4b3f27'),  // tan / brown
    playerKing: createGraphic(['player-king'], '#d4a574', '#8b6f47'),  // tan / brown
    playerPyromaniac: createGraphic(['player-pyromaniac'], '#ff8c42', '#8b4513'),  // orange / saddle brown
    
    // Objects
    tree: createGraphic(['pine-tree'], '#51cf66', '#1a5f1a'),
    rock: createGraphic(['gem'], '#888888', '#333333'),
    stick: createGraphic(
        { shape: 'line', dimensions: { x1: 0, y1: -0.4, x2: 0, y2: 0.4 }, strokeCap: 'round' },
        '#8b6f47',  // brown foreground
        '#4a3a24',  // darker brown outline
        { rotation: 80 }
    ),
    
    // Enemies
    enemyIndicator: createGraphic(['crossed-swords'], '#ff6b6b', '#8b0000'),
    enemyIndicatorDefeated: createGraphic(['crossed-swords'], '#888888', '#444444'),
    enemyMonster: createGraphic(['monster-skull'], '#ff6b6b', '#8b0000'),
    
    // Enemy Types
    enemyRabbit: createGraphic(['rabbit'], '#d4a574', '#8b6f47'),  // tan / brown (neutral, peaceful)
    enemyFox: createGraphic(['fox'], '#ff8c42', '#8b4513'),  // orange / brown (warm, clever)
    enemyWolf: createGraphic(['wolf-head'], '#4a4a4a', '#1a1a1a'),  // dark gray / black (threatening)
    enemySpider: createGraphic(['spider-face'], '#3d1f00', '#000000'),  // dark brown / black (sinister)
    enemySnake: createGraphic(['venomous-snake'], '#51cf66', '#1a5f1a'),  // green / dark green (venomous)
    enemyLion: createGraphic(['lion'], '#d4a574', '#8b6f47'),  // tan / brown (regal, powerful)
    enemyDragon: createGraphic(['dragon'], '#8b0000', '#330000'),  // deep red / black (fierce, boss-level)
    
    // UI Icons
    deathSkull: createGraphic(['skull'], '#ff6b6b', '#8b0000'),
    victoryTrophy: createGraphic(['trophy'], '#51cf66', '#1a5f1a'),
};

// Object type to Graphic mapping
const OBJECT_GRAPHICS = {
    'Rock': GRAPHICS.rock,
    'Tree': GRAPHICS.tree,
    'Stick': GRAPHICS.stick,
};

// Biome decorative icons (simple, no outline)
const BIOME_ICONS = {
    'Forest': ['pine-tree', 'pine-tree'],
    'Meadow': ['flower', 'grass'],
    'Lake': ['droplet', 'droplet'],
    'Mountain': ['mountains', 'mountains'],
    'Plains': ['grass-patch', 'grass-patch'],
};

/**
 * Renders an SVG shape with outline effect.
 * @param {object} shapeDef - Shape definition { shape, dimensions, strokeCap }
 * @param {string} foreground - Fill color
 * @param {string} background - Outline/stroke color
 * @param {number} strokeWidth - Outline thickness
 * @param {number} fontSize - Base size for scaling dimensions
 * @returns {SVGElement} - The rendered SVG element
 */
function renderSVGShape(shapeDef, foreground, background, strokeWidth, fontSize) {
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('width', fontSize);
    svg.setAttribute('height', fontSize);
    svg.setAttribute('viewBox', `${-fontSize/2} ${-fontSize/2} ${fontSize} ${fontSize}`);
    svg.style.display = 'block';
    svg.style.overflow = 'visible';
    
    const { shape, dimensions, strokeCap = 'round' } = shapeDef;
    
    // Helper to create shape element
    const createShapeElement = () => {
        if (shape === 'line') {
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('x1', dimensions.x1 * fontSize);
            line.setAttribute('y1', dimensions.y1 * fontSize);
            line.setAttribute('x2', dimensions.x2 * fontSize);
            line.setAttribute('y2', dimensions.y2 * fontSize);
            line.setAttribute('stroke-linecap', strokeCap);
            return line;
        } else if (shape === 'circle') {
            const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
            circle.setAttribute('cx', 0);
            circle.setAttribute('cy', 0);
            circle.setAttribute('r', dimensions.r * fontSize);
            return circle;
        } else if (shape === 'rect') {
            const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
            const w = dimensions.width * fontSize;
            const h = dimensions.height * fontSize;
            rect.setAttribute('x', -w/2);
            rect.setAttribute('y', -h/2);
            rect.setAttribute('width', w);
            rect.setAttribute('height', h);
            return rect;
        }
    };
    
    // Background/outline layer - drawn with thick stroke
    const bgShape = createShapeElement();
    bgShape.setAttribute('fill', 'none');
    bgShape.setAttribute('stroke', background);
    bgShape.setAttribute('stroke-width', strokeWidth * 2);
    if (shape === 'line') {
        bgShape.setAttribute('stroke-linecap', strokeCap);
    }
    svg.appendChild(bgShape);
    
    // Foreground layer - drawn with fill or thin stroke
    const fgShape = createShapeElement();
    if (shape === 'line') {
        fgShape.setAttribute('fill', 'none');
        fgShape.setAttribute('stroke', foreground);
        fgShape.setAttribute('stroke-width', strokeWidth);
        fgShape.setAttribute('stroke-linecap', strokeCap);
    } else {
        fgShape.setAttribute('fill', foreground);
        fgShape.setAttribute('stroke', 'none');
    }
    svg.appendChild(fgShape);
    
    return svg;
}

/**
 * Renders a Graphic definition as a DOM element.
 * Draws background layer with stroke first, then foreground layer on top.
 * @param {object} graphic - Graphic definition from GRAPHICS
 * @param {number} fontSize - Font size in pixels
 * @returns {HTMLElement} - The rendered graphic element
 */
function renderGraphic(graphic, fontSize = 24) {
    const container = document.createElement('span');
    container.className = 'graphic';
    container.style.fontSize = `${fontSize}px`;
    
    // Build transform string from scale and rotation
    // CSS transforms apply right-to-left, so order is: rotate, scale
    // This means scale is applied first, then rotation
    const transforms = [];
    if (graphic.rotation !== 0) {
        transforms.push(`rotate(${graphic.rotation}deg)`);
    }
    if (graphic.scaleX !== 1 || graphic.scaleY !== 1) {
        transforms.push(`scale(${graphic.scaleX}, ${graphic.scaleY})`);
    }
    if (transforms.length > 0) {
        container.style.transform = transforms.join(' ');
        container.style.transformOrigin = 'center';
    }

    // Route to appropriate renderer based on type
    if (graphic.type === 'shape') {
        // Render SVG shape
        const svg = renderSVGShape(
            graphic.definition,
            graphic.foreground,
            graphic.background,
            graphic.strokeWidth,
            fontSize
        );
        container.appendChild(svg);
    } else {
        // Render font icon(s)
        for (const iconName of graphic.definition) {
            // Background layer (outline/stroke)
            const bgIcon = document.createElement('i');
            bgIcon.className = `ra ra-${iconName} graphic-bg`;
            bgIcon.style.color = graphic.background;
            bgIcon.style.webkitTextStroke = `${graphic.strokeWidth}px ${graphic.background}`;
            container.appendChild(bgIcon);

            // Foreground layer (fill)
            const fgIcon = document.createElement('i');
            fgIcon.className = `ra ra-${iconName} graphic-fg`;
            fgIcon.style.color = graphic.foreground;
            container.appendChild(fgIcon);
        }
    }

    return container;
}

/**
 * Renders a character graphic with the outline effect.
 * Convenience wrapper for character rendering.
 * @param {number} fontSize - Font size in pixels
 * @returns {HTMLElement}
 */
function renderCharacter(fontSize = 24) {
    return renderGraphic(GRAPHICS.player, fontSize);
}

/**
 * Renders a simple RPG-Awesome icon without outline effects.
 * Used for decorative biome icons.
 * @param {string} iconName - RPG-Awesome icon name (without 'ra-' prefix)
 * @param {Array} color - RGB color array [r, g, b] (0.0-1.0)
 * @param {number} fontSize - Font size in pixels
 * @returns {HTMLElement}
 */
function renderSimpleIcon(iconName, color, fontSize = 20) {
    const container = document.createElement('span');
    container.className = 'graphic';
    container.style.fontSize = `${fontSize}px`;
    container.style.display = 'inline-flex';
    container.style.alignItems = 'center';
    container.style.justifyContent = 'center';
    
    const icon = document.createElement('i');
    icon.className = `ra ra-${iconName}`;
    icon.style.color = rgbToCss(color);
    
    container.appendChild(icon);
    return container;
}

/**
 * Renders a character with equipped item overlay.
 * If an equipped item exists, draws it smaller to the right of the character.
 * @param {number} fontSize - Base font size for character
 * @param {number} equipmentScale - Scale factor for equipment (default 0.4)
 * @returns {HTMLElement} Container with character and equipped item
 */
function renderCharacterWithEquipment(fontSize = 24, equipmentScale = 0.4) {
    const container = document.createElement('div');
    container.style.position = 'relative';
    container.style.display = 'inline-flex';
    container.style.alignItems = 'center';
    container.style.justifyContent = 'center';
    
    // Render base character
    const character = renderGraphic(GRAPHICS.player, fontSize);
    container.appendChild(character);
    
    // Check if there's an equipped item
    const equippedName = gameState?.character?.equipped;
    if (equippedName && OBJECT_GRAPHICS[equippedName]) {
        const equipmentEl = document.createElement('div');
        equipmentEl.className = 'equipped-item-icon';
        equipmentEl.style.position = 'absolute';
        equipmentEl.style.right = '-15%';
        equipmentEl.style.bottom = '5%';
        equipmentEl.style.zIndex = '15';
        equipmentEl.style.pointerEvents = 'none';
        equipmentEl.title = `Equipped: ${equippedName}`;
        
        // Render equipment at specified scale
        const equipmentSize = Math.round(fontSize * equipmentScale);
        equipmentEl.appendChild(renderGraphic(OBJECT_GRAPHICS[equippedName], equipmentSize));
        container.appendChild(equipmentEl);
    }
    
    return container;
}

// ===========================================
// Color Utilities
// ===========================================

// Biome colors (RGB 0.0-1.0)
const BIOME_COLORS = {
    'Forest': [0.1, 0.5, 0.1],
    'Meadow': [0.7, 0.9, 0.4],
    'Lake': [0.2, 0.5, 0.9],
    'Mountain': [0.8, 0.8, 0.85],
    'Plains': [0.6, 0.5, 0.35],
};

// Substrate colors (RGB 0.0-1.0)
const SUBSTRATE_COLORS = {
    'Grass': [0.7, 0.9, 0.4],
    'Dirt': [0.6, 0.4, 0.2],
    'Stone': [0.7, 0.7, 0.7],
    'Mud': [0.4, 0.3, 0.2],
    'Water': [0.2, 0.4, 0.9],
    'Brush': [0.2, 0.6, 0.15],
};

// Object colors (RGB 0.0-1.0) - fallback when no icon available
const OBJECT_COLORS = {
    'Rock': [0.3, 0.3, 0.3],
    'Tree': [0.1, 0.6, 0.1],
    'Stick': [0.5, 0.3, 0.1],
};

function getBiomeColor(biome) {
    return BIOME_COLORS[biome] || [0.5, 0.5, 0.5];
}

function getSubstrateColor(substrate) {
    return SUBSTRATE_COLORS[substrate] || [0.5, 0.5, 0.5];
}

function getObjectColor(object) {
    return OBJECT_COLORS[object] || [0.5, 0.5, 0.5];
}

function rgbToCss(rgb) {
    const [r, g, b] = rgb;
    return `rgb(${Math.round(r * 255)}, ${Math.round(g * 255)}, ${Math.round(b * 255)})`;
}

function darkenColor(rgb, factor = 0.8) {
    return rgb.map(c => c * factor);
}

// ===========================================
// Game State
// ===========================================

let gameState = null;
let commandHistory = [];

// Frontend-only overlay state
let displayOverlay = null; // 'death' | 'win' | 'inventory' | 'equip-select' | 'craft-select' | null
let previousViewMode = null;
let equipSelectIndex = 0; // Currently selected item index in equip mode
let craftSelectIndex = 0; // Currently selected recipe index in craft mode
let craftableRecipes = []; // Array of { id: string, name: string }

// Load initial game state
async function loadGameState() {
    try {
        const response = await fetch('/api/state');
        const data = await response.json();
        gameState = data;
        previousViewMode = gameState.core_state.type;
        renderGame();
        updateStatus();
    } catch (error) {
        console.error('Failed to load game state:', error);
        showMessage('Failed to load game state', 'error');
    }
}

// Detect state transitions and show appropriate overlays
function handleStateTransition(commandResponse) {
    const newMode = gameState.core_state.type;
    const message = commandResponse?.message || '';
    
    // Detect death: was in Combat, now in Terrain, message indicates defeat
    if (previousViewMode === 'Combat' && newMode === 'Terrain' && 
        (message.includes('Defeated') || message.includes('💀'))) {
        displayOverlay = 'death';
    }
    
    // Detect win: was in Combat, now in Land, message indicates victory
    if (previousViewMode === 'Combat' && newMode === 'Land' &&
        (message.includes('Victory') || message.includes('⚔️ Victory'))) {
        displayOverlay = 'win';
    }
    
    previousViewMode = newMode;
}

// Execute a command
async function executeCommand(command) {
    if (!command.trim()) return;
    
    // Intercept "e" or "equip" with no arguments - enter equip selection mode
    const trimmedCmd = command.trim().toLowerCase();
    if (trimmedCmd === 'e' || trimmedCmd === 'equip') {
        const inventory = gameState?.character?.inventory || [];
        if (inventory.length === 0) {
            showMessage('Your inventory is empty - nothing to equip', 'error');
            return;
        }
        // Enter equip selection mode
        equipSelectIndex = 0;
        displayOverlay = 'equip-select';
        renderGame();
        showMessage('Use arrow keys to select item, Enter to equip, Escape to cancel', 'success');
        return;
    }
    
    // Intercept "c" or "craft" with no arguments - enter craft selection mode
    if (trimmedCmd === 'c' || trimmedCmd === 'craft') {
        await fetchCraftableRecipes();
        return;
    }

    try {
        const response = await fetch('/api/command', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ command: command.trim() }),
        });

        const data = await response.json();
        gameState = data.game_state;

        // Handle state transitions for overlays
        handleStateTransition(data);

        // Add to history
        commandHistory.push({
            command: command.trim(),
            response: data.message,
            success: data.success,
        });
        if (commandHistory.length > 50) {
            commandHistory.shift();
        }

        renderHistory();
        renderGame();
        updateStatus();
        showMessage(data.message, data.success ? 'success' : 'error');
    } catch (error) {
        console.error('Failed to execute command:', error);
        showMessage('Failed to execute command', 'error');
    }
}

// Parse craftable recipes from "can" command output
function parseCraftableRecipes(message) {
    const recipes = [];
    const lines = message.split('\n');
    
    for (const line of lines) {
        // Look for lines like: "  ✓ recipe_id - Recipe Name"
        const match = line.match(/^\s*✓\s+(\S+)\s+-\s+(.+)$/);
        if (match) {
            recipes.push({
                id: match[1],
                name: match[2].trim()
            });
        }
    }
    
    return recipes;
}

// Fetch craftable recipes using "can" command
async function fetchCraftableRecipes() {
    try {
        const response = await fetch('/api/command', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ command: 'can' }),
        });
        
        const data = await response.json();
        gameState = data.game_state;
        
        // Parse the craftable recipes from response message
        craftableRecipes = parseCraftableRecipes(data.message);
        
        if (craftableRecipes.length === 0) {
            showMessage('No craftable recipes available', 'error');
            return;
        }
        
        craftSelectIndex = 0;
        displayOverlay = 'craft-select';
        renderGame();
        showMessage('Select recipe to craft', 'success');
    } catch (error) {
        showMessage('Failed to fetch craftable recipes', 'error');
    }
}

// Render the game display
function renderGame() {
    const display = document.getElementById('game-display');
    
    // Remove any existing inventory overlay first (it's attached to body, not game-display)
    const existingOverlay = document.getElementById('inventory-overlay');
    if (existingOverlay) {
        existingOverlay.remove();
    }
    
    // Check frontend overlays first (these are shown on top of the actual view)
    if (displayOverlay === 'inventory') {
        // For inventory, render the underlying view first, then show inventory overlay
        renderUnderlyingView(display);
        renderInventoryOverlay();
        } else if (displayOverlay === 'equip-select') {
            // For equip selection, render the underlying view first, then show equip selector
            renderUnderlyingView(display);
            renderEquipSelectOverlay();
        } else if (displayOverlay === 'craft-select') {
            // For craft selection, render the underlying view first, then show craft selector
            renderUnderlyingView(display);
            renderCraftSelectOverlay();
        } else if (displayOverlay === 'death') {
        display.innerHTML = '<div class="screen-view death-screen" id="death-screen"></div>';
        renderDeathScreen();
    } else if (displayOverlay === 'win') {
        display.innerHTML = '<div class="screen-view win-screen" id="win-screen"></div>';
        renderWinScreen();
    } else {
        // Render based on core_state.type (discriminated union)
        renderUnderlyingView(display);
    }
}

// Render the underlying view based on core_state type
function renderUnderlyingView(display) {
    const state = gameState.core_state;
    switch (state.type) {
        case 'Terrain':
            display.innerHTML = '<div class="terrain-view" id="terrain-grid"></div>';
            renderTerrainView(state);
            break;
        case 'Land':
            display.innerHTML = '<div class="land-view" id="land-grid"></div>';
            renderLandView(state);
            break;
        case 'Combat':
            display.innerHTML = '<div class="combat-view" id="combat-view"></div>';
            renderCombatView(state);
            break;
    }
}

// Render death screen
function renderDeathScreen() {
    const screen = document.getElementById('death-screen');
    screen.innerHTML = `
        <div class="screen-icon" id="death-icon"></div>
        <div class="screen-title">Defeated</div>
        <div class="screen-message">You have been defeated in combat!</div>
        <div class="screen-instruction">Press ENTER to continue</div>
    `;
    document.getElementById('death-icon').appendChild(renderGraphic(GRAPHICS.deathSkull, 80));
}

// Render win screen
function renderWinScreen() {
    const screen = document.getElementById('win-screen');
    screen.innerHTML = `
        <div class="screen-icon" id="win-icon"></div>
        <div class="screen-title">Victory</div>
        <div class="screen-message">You have defeated your enemy!</div>
        <div class="screen-instruction">Press ENTER to continue</div>
    `;
    document.getElementById('win-icon').appendChild(renderGraphic(GRAPHICS.victoryTrophy, 80));
}

// Render inventory overlay
function renderInventoryOverlay() {
    // Create overlay container
    const overlay = document.createElement('div');
    overlay.className = 'inventory-overlay';
    overlay.id = 'inventory-overlay';
    
    const inventory = gameState.character.inventory || [];
    const itemsHtml = inventory.length > 0
        ? inventory.map(itemName => {
            const graphic = OBJECT_GRAPHICS[itemName];
            let iconHtml = '';
            if (graphic) {
                const iconContainer = document.createElement('div');
                iconContainer.className = 'inventory-item-icon';
                iconContainer.appendChild(renderGraphic(graphic, 32));
                iconHtml = iconContainer.outerHTML;
            }
            return `
                <div class="inventory-item">
                    ${iconHtml}
                    <div class="inventory-item-name">${itemName}</div>
                </div>
            `;
        }).join('')
        : '<div class="inventory-empty">Your inventory is empty</div>';
    
    overlay.innerHTML = `
        <div class="inventory-panel">
            <div class="inventory-header">
                <div class="inventory-title">Inventory</div>
                <div class="inventory-subtitle">${inventory.length} item${inventory.length !== 1 ? 's' : ''}</div>
            </div>
            <div class="inventory-list">
                ${itemsHtml}
            </div>
            <div class="inventory-instructions">
                <p>Press <code>\`</code> (backtick) or <code>I</code> to close inventory</p>
            </div>
        </div>
    `;
    
    document.body.appendChild(overlay);
}

// Render equip selection overlay
function renderEquipSelectOverlay() {
    // Create overlay container
    const overlay = document.createElement('div');
    overlay.className = 'inventory-overlay';
    overlay.id = 'inventory-overlay';
    
    const inventory = gameState.character.inventory || [];
    
    // Clamp selected index
    if (equipSelectIndex >= inventory.length) {
        equipSelectIndex = inventory.length - 1;
    }
    if (equipSelectIndex < 0) {
        equipSelectIndex = 0;
    }
    
    const itemsHtml = inventory.map((itemName, index) => {
        const graphic = OBJECT_GRAPHICS[itemName];
        let iconHtml = '';
        if (graphic) {
            const iconContainer = document.createElement('div');
            iconContainer.className = 'inventory-item-icon';
            iconContainer.appendChild(renderGraphic(graphic, 32));
            iconHtml = iconContainer.outerHTML;
        }
        const selectedClass = index === equipSelectIndex ? ' selected' : '';
        return `
            <div class="inventory-item${selectedClass}">
                <div class="inventory-item-index">${index}</div>
                ${iconHtml}
                <div class="inventory-item-name">${itemName}</div>
            </div>
        `;
    }).join('');
    
    overlay.innerHTML = `
        <div class="inventory-panel equip-mode">
            <div class="inventory-header">
                <div class="inventory-title">⚔️ Equip Item</div>
                <div class="inventory-subtitle">Select item to equip (${equipSelectIndex + 1}/${inventory.length})</div>
            </div>
            <div class="inventory-list">
                ${itemsHtml}
            </div>
            <div class="inventory-instructions">
                <p>Use <code>Arrow Keys</code> to navigate grid • <code>0-9</code> to jump to item</p>
                <p>Press <code>Enter</code> to equip • <code>Escape</code> or <code>\`</code> to cancel</p>
            </div>
        </div>
    `;
    
    document.body.appendChild(overlay);
}

// Render craft selection overlay
function renderCraftSelectOverlay() {
    const overlay = document.createElement('div');
    overlay.className = 'inventory-overlay';
    overlay.id = 'inventory-overlay';
    
    if (craftSelectIndex >= craftableRecipes.length) craftSelectIndex = craftableRecipes.length - 1;
    if (craftSelectIndex < 0) craftSelectIndex = 0;
    
    const recipesHtml = craftableRecipes.map((recipe, index) => {
        const selectedClass = index === craftSelectIndex ? ' selected' : '';
        return `
            <div class="inventory-item${selectedClass}">
                <div class="inventory-item-index">${index}</div>
                <div class="inventory-item-name">${recipe.name}</div>
            </div>
        `;
    }).join('');
    
    overlay.innerHTML = `
        <div class="inventory-panel equip-mode">
            <div class="inventory-header">
                <div class="inventory-title">🔨 Craft Item</div>
                <div class="inventory-subtitle">Select recipe (${craftSelectIndex + 1}/${craftableRecipes.length})</div>
            </div>
            <div class="inventory-list">
                ${recipesHtml}
            </div>
            <div class="inventory-instructions">
                <p>Use <code>Arrow Keys</code> to navigate grid • <code>0-9</code> to jump</p>
                <p>Press <code>Enter</code> to craft • <code>Escape</code> or <code>\`</code> to cancel</p>
            </div>
        </div>
    `;
    
    document.body.appendChild(overlay);
}

// Render terrain view (5x5 grid)
function renderTerrainView(terrainState) {
    const grid = document.getElementById('terrain-grid');
    const [currentX, currentY] = terrainState.current_land;

    // Build a map for quick land lookup
    const landMap = {};
    terrainState.lands.forEach(land => {
        const key = `${land.coords[0]},${land.coords[1]}`;
        landMap[key] = land;
    });

    for (let y = 0; y < 5; y++) {
        for (let x = 0; x < 5; x++) {
            const cell = document.createElement('div');
            cell.className = 'terrain-cell';
            
            if (x === currentX && y === currentY) {
                cell.classList.add('current');
            }

            const landKey = `${x},${y}`;
            const land = landMap[landKey];
            
            if (land) {
                cell.style.backgroundColor = rgbToCss(getBiomeColor(land.biome));
                
                // Add biome decorative icons
                const biomeIcons = BIOME_ICONS[land.biome];
                if (biomeIcons) {
                    const iconContainer = document.createElement('div');
                    iconContainer.className = 'biome-icon-overlay';
                    const darkerColor = darkenColor(getBiomeColor(land.biome));
                    
                    biomeIcons.forEach((iconName, index) => {
                        const iconEl = renderSimpleIcon(iconName, darkerColor, 32);
                        // Add negative margin to subsequent icons for overlap
                        if (index > 0) {
                            iconEl.style.marginLeft = '-16px';
                        }
                        iconContainer.appendChild(iconEl);
                    });
                    
                    cell.appendChild(iconContainer);
                }
                
                // Build tooltip with enemy info if present
                let title = `Land (${x}, ${y}): ${land.biome}`;
                if (land.enemy) {
                    const enemyStatus = land.enemy.is_defeated ? 'Defeated' : 'Alive';
                    title += ` | Enemy: ${enemyStatus} (HP: ${land.enemy.health}/${land.enemy.max_health}, ATK: ${land.enemy.attack})`;
                }
                cell.title = title;
                
                // Add enemy indicator if enemy exists
                if (land.enemy) {
                    const isDefeated = land.enemy.is_defeated;
                    const enemyIndicator = document.createElement('div');
                    enemyIndicator.className = 'enemy-indicator' + (isDefeated ? ' defeated' : '');
                    enemyIndicator.title = `Enemy: ${isDefeated ? 'Defeated' : 'Alive'} (HP: ${land.enemy.health}/${land.enemy.max_health}, ATK: ${land.enemy.attack})`;
                    
                    const graphic = isDefeated ? GRAPHICS.enemyIndicatorDefeated : GRAPHICS.enemyIndicator;
                    enemyIndicator.appendChild(renderGraphic(graphic, 20));
                    cell.appendChild(enemyIndicator);
                }
            } else {
                cell.style.backgroundColor = '#1a1a1a';
                cell.title = `Ungenerated (${x}, ${y})`;
            }

            // Add character if on this land
            if (x === currentX && y === currentY) {
                const charContainer = document.createElement('div');
                charContainer.className = 'character-sprite';
                charContainer.style.display = 'flex';
                charContainer.style.alignItems = 'center';
                charContainer.style.justifyContent = 'center';
                charContainer.title = 'Character';
                charContainer.appendChild(renderCharacterWithEquipment(36, 0.5)); // 50% for terrain view
                cell.appendChild(charContainer);
            }

            grid.appendChild(cell);
        }
    }
}

// Render land view (8x8 grid)
function renderLandView(landState) {
    const grid = document.getElementById('land-grid');
    const [tileX, tileY] = landState.current_tile;
    const tiles = landState.tiles;

    for (let y = 0; y < 8; y++) {
        for (let x = 0; x < 8; x++) {
            const cell = document.createElement('div');
            cell.className = 'tile-cell';
            
            if (x === tileX && y === tileY) {
                cell.classList.add('current');
            }

            const tile = tiles[y][x];
            cell.style.backgroundColor = rgbToCss(getSubstrateColor(tile.substrate));
            
            // Render object if present
            if (tile.objects && tile.objects.length > 0) {
                const objectName = tile.objects[0];
                const objectGraphic = OBJECT_GRAPHICS[objectName];
                
                if (objectGraphic) {
                    const objectEl = document.createElement('div');
                    objectEl.className = 'tile-object-icon';
                    objectEl.title = objectName;
                    objectEl.appendChild(renderGraphic(objectGraphic, 20));
                    cell.appendChild(objectEl);
                } else {
                    // Fallback to color fill if no graphic defined
                    cell.style.backgroundColor = rgbToCss(getObjectColor(objectName));
                }
            }
            
            // Render character if on this tile
            if (x === tileX && y === tileY) {
                const charContainer = document.createElement('div');
                charContainer.className = 'character-sprite';
                charContainer.style.display = 'flex';
                charContainer.style.alignItems = 'center';
                charContainer.style.justifyContent = 'center';
                charContainer.title = 'Character';
                charContainer.appendChild(renderCharacterWithEquipment(24));
                cell.appendChild(charContainer);
            }
            
            const objectsStr = tile.objects.length > 0 ? ' + ' + tile.objects[0] : '';
            cell.title = `Tile (${x}, ${y}): ${tile.substrate}${objectsStr}`;

            grid.appendChild(cell);
        }
    }
}

// Note: Color helper functions (getBiomeColor, getSubstrateColor, getObjectColor, rgbToCss)
// are now defined at the top of the script in the Color Utilities section.

// Render combat view
function renderCombatView(combatState) {
    const combatView = document.getElementById('combat-view');
    
    const player = combatState.player;
    const enemy = combatState.enemy;
    const playerMaxHealth = gameState.character.max_health;
    const enemyMaxHealth = combatState.enemy_max_health;

    const playerHealthPercent = Math.max(0, (player.health / playerMaxHealth) * 100);
    const enemyHealthPercent = Math.max(0, (enemy.health / enemyMaxHealth) * 100);

    const getHealthClass = (percent) => percent > 60 ? 'high' : (percent > 30 ? 'medium' : 'low');

    // Build the combat view structure
    combatView.innerHTML = `
        <div class="combat-header">
            <div class="combat-title">Combat</div>
            <div class="combat-round">Round ${combatState.round}</div>
        </div>
        <div class="combat-panels">
            <div class="combatant-panel player">
                <div class="combatant-name">Player</div>
                <div class="combatant-sprite" id="player-sprite"></div>
                <div class="combatant-stats">
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">HP</span>
                        <span class="combat-stat-value">${player.health}/${playerMaxHealth}</span>
                    </div>
                    <div class="combat-health-bar-container">
                        <div class="combat-health-bar ${getHealthClass(playerHealthPercent)}" style="width: ${playerHealthPercent}%"></div>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">ATK</span>
                        <span class="combat-stat-value">${player.attack}</span>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">DEF</span>
                        <span class="combat-stat-value">${player.defense}</span>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">ACC</span>
                        <span class="combat-stat-value">${player.accuracy}</span>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">EVA</span>
                        <span class="combat-stat-value">${player.evasion}</span>
                    </div>
                </div>
            </div>
            <div class="combatant-panel enemy">
                <div class="combatant-name">${combatState.enemy_type}</div>
                <div class="combatant-sprite" id="enemy-sprite"></div>
                <div class="combatant-stats">
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">HP</span>
                        <span class="combat-stat-value">${enemy.health}/${enemyMaxHealth}</span>
                    </div>
                    <div class="combat-health-bar-container">
                        <div class="combat-health-bar ${getHealthClass(enemyHealthPercent)}" style="width: ${enemyHealthPercent}%"></div>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">ATK</span>
                        <span class="combat-stat-value">${enemy.attack}</span>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">DEF</span>
                        <span class="combat-stat-value">${enemy.defense}</span>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">ACC</span>
                        <span class="combat-stat-value">${enemy.accuracy}</span>
                    </div>
                    <div class="combat-stat-row">
                        <span class="combat-stat-label">EVA</span>
                        <span class="combat-stat-value">${enemy.evasion}</span>
                    </div>
                </div>
            </div>
        </div>
        <div class="combat-actions">
            <h3>Combat Commands</h3>
            <p><code>A</code> or <code>ATTACK</code> - Attack the enemy</p>
            <p><code>X</code> / <code>EXIT</code> - Flee combat</p>
        </div>
    `;

    // Render sprites using the Graphic system
    const playerSpriteEl = document.getElementById('player-sprite');
    playerSpriteEl.style.display = 'flex';
    playerSpriteEl.style.alignItems = 'center';
    playerSpriteEl.style.justifyContent = 'center';
    playerSpriteEl.appendChild(renderCharacterWithEquipment(60));

    const enemySpriteEl = document.getElementById('enemy-sprite');
    enemySpriteEl.style.display = 'flex';
    enemySpriteEl.style.alignItems = 'center';
    enemySpriteEl.style.justifyContent = 'center';
    
    // Map enemy type to graphic
    const enemyTypeGraphicMap = {
        'Rabbit': GRAPHICS.enemyRabbit,
        'Fox': GRAPHICS.enemyFox,
        'Wolf': GRAPHICS.enemyWolf,
        'Spider': GRAPHICS.enemySpider,
        'Snake': GRAPHICS.enemySnake,
        'Lion': GRAPHICS.enemyLion,
        'Dragon': GRAPHICS.enemyDragon,
    };
    
    const enemyGraphic = enemyTypeGraphicMap[combatState.enemy_type] || GRAPHICS.enemyMonster;
    enemySpriteEl.appendChild(renderGraphic(enemyGraphic, 60));
}

// Update status bar
function updateStatus() {
    const coreState = gameState.core_state;
    document.getElementById('view-mode').textContent = coreState.type;
    
    // Get current land based on state type
    let currentLand;
    if (coreState.type === 'Terrain') {
        currentLand = coreState.current_land;
    } else if (coreState.type === 'Land') {
        currentLand = coreState.land_coords;
    } else if (coreState.type === 'Combat') {
        currentLand = coreState.land_coords;
    }
    
    document.getElementById('current-land').textContent = 
        `(${currentLand[0]}, ${currentLand[1]})`;

    // Show biome in Terrain view
    const biomeItem = document.getElementById('biome-item');
    if (coreState.type === 'Terrain') {
        // Find the current land in the lands array
        const land = coreState.lands.find(l => 
            l.coords[0] === currentLand[0] && l.coords[1] === currentLand[1]
        );
        if (land) {
            document.getElementById('current-biome').textContent = land.biome;
            biomeItem.style.display = 'flex';
        } else {
            biomeItem.style.display = 'none';
        }
    } else {
        biomeItem.style.display = 'none';
    }

    // Update tile information if available (Land view only)
    const tileInfoSection = document.getElementById('tile-info-section');
    if (coreState.type === 'Land') {
        const [tileX, tileY] = coreState.current_tile;
        const tile = coreState.tiles[tileY][tileX];
        
        document.getElementById('tile-biome').textContent = coreState.biome;
        document.getElementById('tile-substrate').textContent = tile.substrate;
        
        if (tile.objects && tile.objects.length > 0) {
            document.getElementById('tile-objects').textContent = tile.objects.join(', ');
        } else {
            document.getElementById('tile-objects').textContent = 'None';
        }
        
        tileInfoSection.style.display = 'flex';
    } else {
        tileInfoSection.style.display = 'none';
    }

    // Update character stats
    if (gameState.character) {
        const char = gameState.character;
        const healthPercent = (char.health / char.max_health) * 100;
        document.getElementById('character-health').textContent = `${char.health}/${char.max_health} (${Math.round(healthPercent)}%)`;
        
        // Display attack with bonus breakdown if applicable
        const baseAttack = 5; // Current MVP hardcoded base attack value
        const totalAttack = char.attack;
        const attackBonus = totalAttack - baseAttack;
        
        if (attackBonus > 0) {
            document.getElementById('character-attack').textContent = `${totalAttack} (${baseAttack} + ${attackBonus})`;
        } else {
            document.getElementById('character-attack').textContent = `${totalAttack}`;
        }
        
        // Display defense with bonus breakdown if applicable
        const baseDefense = 0; // Base defense
        const totalDefense = char.defense;
        const defenseBonus = totalDefense - baseDefense;
        
        if (defenseBonus > 0) {
            document.getElementById('character-defense').textContent = `${totalDefense} (${baseDefense} + ${defenseBonus})`;
        } else {
            document.getElementById('character-defense').textContent = `${totalDefense}`;
        }
        
        // Display accuracy with bonus breakdown if applicable
        const baseAccuracy = 10; // Base accuracy (100% hit chance)
        const totalAccuracy = char.accuracy;
        const accuracyBonus = totalAccuracy - baseAccuracy;
        
        if (accuracyBonus > 0) {
            document.getElementById('character-accuracy').textContent = `${totalAccuracy} (${baseAccuracy} + ${accuracyBonus})`;
        } else {
            document.getElementById('character-accuracy').textContent = `${totalAccuracy}`;
        }
        
        // Display evasion with bonus breakdown if applicable
        const baseEvasion = 0; // Base evasion
        const totalEvasion = char.evasion;
        const evasionBonus = totalEvasion - baseEvasion;
        
        if (evasionBonus > 0) {
            document.getElementById('character-evasion').textContent = `${totalEvasion} (${baseEvasion} + ${evasionBonus})`;
        } else {
            document.getElementById('character-evasion').textContent = `${totalEvasion}`;
        }

        // Update health bar
        const healthBar = document.getElementById('health-bar');
        healthBar.style.width = `${healthPercent}%`;

        // Set health bar color based on percentage
        healthBar.className = 'health-bar';
        if (healthPercent > 60) {
            healthBar.classList.add('high');
        } else if (healthPercent > 30) {
            healthBar.classList.add('medium');
        } else {
            healthBar.classList.add('low');
        }
    }

    // Update command input placeholder based on view mode
    const commandInput = document.getElementById('command-input');
    if (coreState.type === 'Combat') {
        commandInput.placeholder = 'Enter command (A/ATTACK, X/FLEE, E to equip, ` for inventory, H/HELP)';
    } else {
        commandInput.placeholder = 'Enter command (M <dir>/X, E to equip, ` for inventory, H/HELP)';
    }

    // Update help section based on view mode
    const helpContent = document.getElementById('help-content');
    if (coreState.type === 'Combat') {
        helpContent.innerHTML = `
            <p><code>A</code> / <code>ATTACK</code> - Attack the enemy</p>
            <p><code>X</code> / <code>EXIT</code> - Flee combat</p>
            <p><code>E</code> - Open equip selector</p>
            <p><code>\`</code> - Toggle inventory</p>
            <p><code>H</code> - Help</p>
        `;
    } else if (coreState.type === 'Land') {
        helpContent.innerHTML = `
            <p><code>Arrow Keys</code> or <code>M &lt;dir&gt;</code> - Move (u/d/l/r)</p>
            <p><code>X</code> / <code>EXIT</code> - Exit Land</p>
            <p><code>E</code> - Open equip selector</p>
            <p><code>C</code> - Open craft selector</p>
            <p><code>\`</code> - Toggle inventory</p>
            <p><code>H</code> - Help</p>
        `;
    } else {
        helpContent.innerHTML = `
            <p><code>Arrow Keys</code> or <code>M &lt;dir&gt;</code> - Move (u/d/l/r)</p>
            <p><code>X</code> / <code>ENTER</code> - Enter Land</p>
            <p><code>E</code> - Open equip selector</p>
            <p><code>\`</code> - Toggle inventory</p>
            <p><code>H</code> - Help</p>
        `;
    }
}

// Render command history
function renderHistory() {
    const historyList = document.getElementById('history-list');
    historyList.innerHTML = '';

    if (commandHistory.length === 0) {
        historyList.innerHTML = '<div style="color: #888; font-style: italic;">No commands yet</div>';
        return;
    }

    // Show most recent first
    const reversed = [...commandHistory].reverse();
    reversed.forEach(item => {
        const div = document.createElement('div');
        
        // Determine log type for color coding
        const response = item.response.toLowerCase();
        let logType = 'mundane'; // Default grey
        
        if (response.includes('⚔️') || response.includes('combat') || response.includes('attack') || response.includes('flee') || response.includes('victory') || response.includes('defeated')) {
            logType = 'combat'; // Red for combat
        }
        
        div.className = `history-item ${logType}`;
        div.innerHTML = `
            <div class="history-command">${item.command}</div>
            <div class="history-response ${item.success ? 'success' : 'error'}">${item.response}</div>
        `;
        historyList.appendChild(div);
    });
}

// Show message (permanently visible)
function showMessage(text, type) {
    const messageEl = document.getElementById('message');
    messageEl.textContent = text;
    messageEl.className = `message ${type}`;
    // Message stays visible permanently - no auto-hide
}

// Set up event listeners
document.getElementById('command-button').addEventListener('click', () => {
    const input = document.getElementById('command-input');
    executeCommand(input.value);
    input.value = '';
});

document.getElementById('command-input').addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        const input = document.getElementById('command-input');
        executeCommand(input.value);
        input.value = '';
    }
});

// Global key handler for special keys (arrow keys, backtick, etc.)
// This handler gates commands based on current game state to ensure consistent behavior
document.addEventListener('keydown', (e) => {
    if (!gameState) return;
    
    // Backtick behavior depends on current overlay
    if (e.key === '`') {
        e.preventDefault();
        if (displayOverlay === 'equip-select') {
            // In equip mode, backtick cancels
            displayOverlay = null;
            renderGame();
            showMessage('Equip cancelled', 'error');
        } else if (displayOverlay === 'inventory') {
            // Close inventory
            displayOverlay = null;
            renderGame();
        } else {
            // Open inventory
            displayOverlay = 'inventory';
            renderGame();
        }
        return;
    }
    
    // 'I' key also closes inventory/equip-select if open
    if (e.key === 'i' || e.key === 'I') {
        if (displayOverlay === 'inventory' || displayOverlay === 'equip-select') {
            e.preventDefault();
            const wasEquipMode = displayOverlay === 'equip-select';
            displayOverlay = null;
            renderGame();
            if (wasEquipMode) {
                showMessage('Equip cancelled', 'error');
            }
        }
        return;
    }
    
    // When inventory is open, only backtick/I is accepted (handled above)
    // All other keys are ignored
    if (displayOverlay === 'inventory') {
        // Prevent arrow keys from doing anything else
        if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
            e.preventDefault();
        }
        return;
    }
    
    // When equip selection is active, handle navigation and confirmation
    if (displayOverlay === 'equip-select') {
        const inventory = gameState?.character?.inventory || [];
        const ITEMS_PER_ROW = 5;
        
        // Number keys 0-9 for instant selection
        if (e.key >= '0' && e.key <= '9') {
            e.preventDefault();
            const targetIndex = parseInt(e.key);
            if (targetIndex < inventory.length) {
                equipSelectIndex = targetIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            // Move up one row (subtract 5)
            const newIndex = equipSelectIndex - ITEMS_PER_ROW;
            if (newIndex >= 0) {
                equipSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            // Move down one row (add 5)
            const newIndex = equipSelectIndex + ITEMS_PER_ROW;
            if (newIndex < inventory.length) {
                equipSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowLeft') {
            e.preventDefault();
            // Move left within row (subtract 1, stay in same row)
            const currentRow = Math.floor(equipSelectIndex / ITEMS_PER_ROW);
            const newIndex = equipSelectIndex - 1;
            const newRow = Math.floor(newIndex / ITEMS_PER_ROW);
            if (newIndex >= 0 && newRow === currentRow) {
                equipSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowRight') {
            e.preventDefault();
            // Move right within row (add 1, stay in same row)
            const currentRow = Math.floor(equipSelectIndex / ITEMS_PER_ROW);
            const newIndex = equipSelectIndex + 1;
            const newRow = Math.floor(newIndex / ITEMS_PER_ROW);
            if (newIndex < inventory.length && newRow === currentRow) {
                equipSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'Enter') {
            e.preventDefault();
            // Execute the equip command with selected index
            displayOverlay = null;
            executeCommand(`e ${equipSelectIndex}`);
        } else if (e.key === 'Escape') {
            e.preventDefault();
            displayOverlay = null;
            renderGame();
            showMessage('Equip cancelled', 'error');
        }
        return;
    }
    
    // When craft selection is active, handle navigation and confirmation
    if (displayOverlay === 'craft-select') {
        const ITEMS_PER_ROW = 5;
        
        // Number keys 0-9 for instant selection
        if (e.key >= '0' && e.key <= '9') {
            e.preventDefault();
            const targetIndex = parseInt(e.key);
            if (targetIndex < craftableRecipes.length) {
                craftSelectIndex = targetIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            // Move up one row (subtract 5)
            const newIndex = craftSelectIndex - ITEMS_PER_ROW;
            if (newIndex >= 0) {
                craftSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            // Move down one row (add 5)
            const newIndex = craftSelectIndex + ITEMS_PER_ROW;
            if (newIndex < craftableRecipes.length) {
                craftSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowLeft') {
            e.preventDefault();
            // Move left within row (subtract 1, stay in same row)
            const currentRow = Math.floor(craftSelectIndex / ITEMS_PER_ROW);
            const newIndex = craftSelectIndex - 1;
            const newRow = Math.floor(newIndex / ITEMS_PER_ROW);
            if (newIndex >= 0 && newRow === currentRow) {
                craftSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'ArrowRight') {
            e.preventDefault();
            // Move right within row (add 1, stay in same row)
            const currentRow = Math.floor(craftSelectIndex / ITEMS_PER_ROW);
            const newIndex = craftSelectIndex + 1;
            const newRow = Math.floor(newIndex / ITEMS_PER_ROW);
            if (newIndex < craftableRecipes.length && newRow === currentRow) {
                craftSelectIndex = newIndex;
                renderGame();
            }
        } else if (e.key === 'Enter') {
            e.preventDefault();
            // Execute the craft command with selected recipe
            const recipe = craftableRecipes[craftSelectIndex];
            displayOverlay = null;
            executeCommand(`craft ${recipe.id}`);
        } else if (e.key === 'Escape') {
            e.preventDefault();
            displayOverlay = null;
            renderGame();
            showMessage('Craft cancelled', 'error');
        }
        return;
    }
    
    // When death/win screen is showing, only Enter dismisses it (frontend-only)
    if (displayOverlay === 'death' || displayOverlay === 'win') {
        if (e.key === 'Enter') {
            e.preventDefault();
            displayOverlay = null;
            renderGame();
        }
        // Prevent arrow keys from doing anything
        if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) {
            e.preventDefault();
        }
        return;
    }
    
    // Normal gameplay: arrow keys trigger movement
    const arrowKeyMap = {
        'ArrowUp': 'm u',
        'ArrowDown': 'm d',
        'ArrowLeft': 'm l',
        'ArrowRight': 'm r',
    };
    
    const command = arrowKeyMap[e.key];
    if (command) {
        e.preventDefault();
        executeCommand(command);
    }
});

// Focus command input on load
document.getElementById('command-input').focus();

// Load initial state
loadGameState();
