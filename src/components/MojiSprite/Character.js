// ─────────────────────────────────────────────────────────────
// Character — clase para definir personajes de forma escalable.
//
// Cada personaje recibe:
//   - name: identificador del personaje
//   - sequences: objeto { estado: ruta_PNG | { src, ...overrides } }
//
// La clase aplica configuración por capas (en orden de prioridad):
//   1. DEFAULTS         → base común para cualquier estado
//   2. STATE_PRESETS    → ajustes típicos según el estado (loop, fps)
//   3. overrides inline → ajustes específicos del personaje
// ─────────────────────────────────────────────────────────────

// Configuración base — aplica a todos los estados de todos los personajes
const DEFAULTS = {
  frames: 36,
  cols: 6,
  rows: 6,
  frameWidth: 130,
  frameHeight: 100,
  fps: 10,
  loop: false,
  restFrame: 0,
};

// Comportamiento típico por estado — sobrescribe DEFAULTS
const STATE_PRESETS = {
  default:   {},                          // expresión de reposo (cada 4 min)
  click:     { fps: 12 },                 // reacción al clic (un poco más rápida)
  work:      { loop: true, fps: 8 },      // estado de enfoque (loop)
  celebrate: { fps: 14 },                 // celebración (rápida)
  sad:       { loop: true, fps: 6 },      // melancolía (loop lento)
  pet:       { fps: 16 },                 // caricia (rápida y corta)
  hydration: { loop: true, fps: 12 },     // recordatorio de agua (loop)
};

export class Character {
  constructor(name, sequences) {
    this.name = name;
    this.states = {};

    for (const [state, value] of Object.entries(sequences)) {
      const isObject = typeof value === 'object' && value !== null;
      const src = isObject ? value.src : value;
      const inlineOverrides = isObject
        ? Object.fromEntries(Object.entries(value).filter(([k]) => k !== 'src'))
        : {};

      this.states[state] = {
        ...DEFAULTS,
        ...(STATE_PRESETS[state] || {}),
        ...inlineOverrides,
        src,
      };
    }
  }
}
