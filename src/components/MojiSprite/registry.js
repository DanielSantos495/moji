import { Character } from './Character';

// PNGs de cada personaje
import kaelDefault from '../../characters/kael/default.png';
import kaelClick from '../../characters/kael/click.png';

// ─────────────────────────────────────────────────────────────
// Crear un personaje:
//
//   new Character('nombre', { estado: rutaPNG, ... });
//
// Si un estado necesita configuración distinta, pásala inline:
//
//   new Character('nombre', {
//     default: rutaDefault,
//     click: { src: rutaClick, fps: 14 },
//   });
//
// Para escalar: importa los PNGs, crea la instancia, agrégala al REGISTRY.
// ─────────────────────────────────────────────────────────────

const kael = new Character('kael', {
  default: kaelDefault,
  click: kaelClick,
});

// Ejemplo futuro:
// import otroDefault from '../../characters/otro/default.png';
// import otroClick from '../../characters/otro/click.png';
// const otro = new Character('otro', { default: otroDefault, click: otroClick });

export const REGISTRY = {
  [kael.name]: kael.states,
  // [otro.name]: otro.states,
};

export const DEFAULT_CHARACTER = 'kael';
export const DEFAULT_STATE = 'default';
export const CLICK_STATE = 'click';
