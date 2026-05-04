import kaelDefault from '../../characters/kael/default.png';

// ─────────────────────────────────────────────────────────────
// REGISTRY — única fuente de verdad para todas las animaciones.
//
// Para agregar una animación nueva:
//   1. Coloca el PNG en src/characters/<personaje>/
//   2. Importa el PNG arriba
//   3. Agrega una entrada aquí con sus parámetros
// ─────────────────────────────────────────────────────────────

export const REGISTRY = {
  kael: {
    // Animación principal — se dispara al cargar y al hacer clic, no hace loop
    default: {
      src: kaelDefault,
      frames: 36,
      cols: 6,
      rows: 6,
      frameWidth: 142,
      frameHeight: 92,
      fps: 12,
      loop: false,      // Plays once per trigger
      restFrame: 0,     // Frame que se muestra en reposo (0 = primer frame)
    },

    // Estados futuros — agrega el PNG y descomenta cuando estén listos:
    // work:      { src: kaelWork,      frames: 24, cols: 6, rows: 4, frameWidth: 542, frameHeight: 332, fps: 10, loop: true },
    // celebrate: { src: kaelCelebrate, frames: 18, cols: 6, rows: 3, frameWidth: 542, frameHeight: 332, fps: 14, loop: false },
    // sad:       { src: kaelSad,       frames: 30, cols: 6, rows: 5, frameWidth: 542, frameHeight: 332, fps: 8,  loop: true },
    // pet:       { src: kaelPet,       frames: 12, cols: 6, rows: 2, frameWidth: 542, frameHeight: 332, fps: 16, loop: false },
    // hydration: { src: kaelHydration, frames: 20, cols: 5, rows: 4, frameWidth: 542, frameHeight: 332, fps: 12, loop: true },
  },
};

export const DEFAULT_CHARACTER = 'kael';
export const DEFAULT_STATE = 'default';
