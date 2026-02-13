// Load and display all recipes

let recipesData = null;

// Load recipes on page load
document.addEventListener('DOMContentLoaded', () => {
    loadRecipes();
});

// Load all recipes from the API
async function loadRecipes() {
    try {
        const response = await fetch('/api/recipes');
        recipesData = await response.json();
        renderRecipes();
    } catch (error) {
        console.error('Failed to load recipes:', error);
        document.getElementById('recipe-viewer-content').innerHTML = `
            <div style="text-align: center; padding: 40px; color: #ff6b6b;">
                Failed to load recipes. Please try again.
            </div>
        `;
    }
}

// Render all recipes
function renderRecipes() {
    const content = document.getElementById('recipe-viewer-content');
    
    const html = `
        ${renderSimpleRecipes(recipesData.simple_recipes)}
        ${renderComponentRecipes(recipesData.component_recipes)}
        ${renderCompositeRecipes(recipesData.composite_recipes)}
    `;
    
    content.innerHTML = html;
}

// Render simple recipes section
function renderSimpleRecipes(recipes) {
    if (recipes.length === 0) {
        return `
            <div class="recipe-section">
                <h2 class="recipe-section-title">Simple Recipes</h2>
                <div class="empty-message">No simple recipes registered.</div>
            </div>
        `;
    }
    
    const recipeCards = recipes.map(recipe => {
        const inputsHtml = recipe.inputs.map(input => 
            `<div class="recipe-input-item">${input.quantity}x ${input.item_id}</div>`
        ).join('');
        
        const requirementsHtml = buildRequirementsHtml(recipe.tool, recipe.world_object);
        
        return `
            <div class="recipe-card">
                <div class="recipe-card-header">
                    <div class="recipe-name">${recipe.name}</div>
                    <div class="recipe-id">${recipe.id}</div>
                </div>
                <div class="recipe-body">
                    <div class="recipe-row">
                        <span class="recipe-label">Inputs:</span>
                        <div class="recipe-inputs-list">
                            ${inputsHtml || '<span style="color: #888;">None</span>'}
                        </div>
                    </div>
                    <div class="recipe-row">
                        <span class="recipe-label">Output:</span>
                        <span class="recipe-value">${recipe.output_quantity}x ${recipe.output}</span>
                    </div>
                </div>
                ${requirementsHtml}
            </div>
        `;
    }).join('');
    
    return `
        <div class="recipe-section">
            <h2 class="recipe-section-title">🔨 Simple Recipes</h2>
            <div class="recipe-section-description">
                Basic crafting recipes that transform raw materials into processed items.
                Examples: Mining ore, smelting metal, harvesting resources.
            </div>
            <div class="recipe-grid">
                ${recipeCards}
            </div>
        </div>
    `;
}

// Render component recipes section
function renderComponentRecipes(recipes) {
    if (recipes.length === 0) {
        return `
            <div class="recipe-section">
                <h2 class="recipe-section-title">Component Recipes</h2>
                <div class="empty-message">No component recipes registered.</div>
            </div>
        `;
    }
    
    const recipeCards = recipes.map(recipe => {
        const requirementsHtml = buildRequirementsHtml(recipe.tool, recipe.world_object);
        
        return `
            <div class="recipe-card">
                <div class="recipe-card-header">
                    <div class="recipe-name">${recipe.name}</div>
                    <div class="recipe-id">${recipe.id}</div>
                </div>
                <div class="recipe-body">
                    <div class="recipe-row">
                        <span class="recipe-label">Input:</span>
                        <span class="recipe-value">1x Submaterial (matching component kind)</span>
                    </div>
                    <div class="recipe-row">
                        <span class="recipe-label">Output:</span>
                        <span class="recipe-value">1x ${recipe.output} (Component)</span>
                    </div>
                </div>
                ${requirementsHtml}
            </div>
        `;
    }).join('');
    
    return `
        <div class="recipe-section">
            <h2 class="recipe-section-title">⚙️ Component Recipes</h2>
            <div class="recipe-section-description">
                Recipes that craft specialized components from submaterials.
                Examples: Blade from flint, Handle from stick, Head from bone.
            </div>
            <div class="recipe-grid">
                ${recipeCards}
            </div>
        </div>
    `;
}

// Render composite recipes section
function renderCompositeRecipes(recipes) {
    if (recipes.length === 0) {
        return `
            <div class="recipe-section">
                <h2 class="recipe-section-title">Composite Recipes</h2>
                <div class="empty-message">No composite recipes registered.</div>
            </div>
        `;
    }
    
    const recipeCards = recipes.map(recipe => {
        const requirementsHtml = buildRequirementsHtml(recipe.tool, recipe.world_object);
        
        return `
            <div class="recipe-card">
                <div class="recipe-card-header">
                    <div class="recipe-name">${recipe.name}</div>
                    <div class="recipe-id">${recipe.id}</div>
                </div>
                <div class="recipe-body">
                    <div class="recipe-row">
                        <span class="recipe-label">Inputs:</span>
                        <span class="recipe-value">Components (matching item definition slots)</span>
                    </div>
                    <div class="recipe-row">
                        <span class="recipe-label">Output:</span>
                        <span class="recipe-value">1x ${recipe.output} (Complete Item)</span>
                    </div>
                </div>
                ${requirementsHtml}
            </div>
        `;
    }).join('');
    
    return `
        <div class="recipe-section">
            <h2 class="recipe-section-title">🛠️ Composite Recipes</h2>
            <div class="recipe-section-description">
                Advanced recipes that assemble complete items from components.
                Examples: Axe from blade + handle, Armor from multiple plates.
            </div>
            <div class="recipe-grid">
                ${recipeCards}
            </div>
        </div>
    `;
}

// Build requirements HTML for a recipe
function buildRequirementsHtml(tool, worldObject) {
    const requirements = [];
    
    if (tool) {
        requirements.push(`
            <div class="recipe-requirement">
                <span class="requirement-icon">🔧</span>
                <span class="requirement-text">Tool: ${tool.tool_type} (Min Quality: ${tool.min_quality})</span>
            </div>
        `);
    }
    
    if (worldObject) {
        let worldObjText = '';
        if (worldObject.kind) {
            worldObjText = `World Object: ${worldObject.kind}`;
        } else if (worldObject.required_tags && worldObject.required_tags.length > 0) {
            worldObjText = `World Object with tags: ${worldObject.required_tags.join(', ')}`;
        }
        
        if (worldObjText) {
            requirements.push(`
                <div class="recipe-requirement">
                    <span class="requirement-icon">🏗️</span>
                    <span class="requirement-text">${worldObjText}</span>
                </div>
            `);
        }
    }
    
    if (requirements.length === 0) {
        return '';
    }
    
    return `
        <div class="recipe-requirements">
            ${requirements.join('')}
        </div>
    `;
}
