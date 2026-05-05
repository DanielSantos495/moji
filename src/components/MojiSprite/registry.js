import kaelDefault from '../../characters/kael/default.png';
import click from '../../characters/kael/click.png';

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
    // Expresión típica de Kael — se reproduce cada 4 minutos automáticamente
    default: {
      src: kaelDefault,
      frames: 36,       // ← ajusta si es diferente
      cols: 6,          // ← ajusta según el grid del PNG (2430 / cols = frameWidth)
      rows: 6,          // ← ajusta según el grid del PNG (1830 / rows = frameHeight)
      frameWidth: 405,  // 2430 / 6
      frameHeight: 305, // 1830 / 6
      fps: 10,
      loop: false,
      restFrame: 0,
    },

    // Animación al hacer clic — se dispara una vez por clic
    click: {
      src: click,
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
export const CLICK_STATE = 'click';
