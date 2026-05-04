import { useRef, useEffect } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import SpritePlayer from './SpritePlayer';
import { REGISTRY, DEFAULT_CHARACTER, DEFAULT_STATE } from './registry';

// API pública: <MojiSprite character="kael" state="default" />
//
// Triggers de animación:
//   - Al cargar: reproduce automáticamente
//   - Al hacer hover (cursor-entered): evento nativo de Tauri, funciona
//     sin importar si la app tiene el foco o no
//   - Si ya está reproduciéndose, ignora el trigger hasta que termine

export default function MojiSprite({
  character = DEFAULT_CHARACTER,
  state = DEFAULT_STATE,
  onStateEnd,
}) {
  const playerRef = useRef(null);

  const config =
    REGISTRY[character]?.[state] ??
    REGISTRY[DEFAULT_CHARACTER][DEFAULT_STATE];

  // Dispara al cargar
  useEffect(() => {
    playerRef.current?.play();
  }, [state]);

  // Escucha el cursor a nivel OS — funciona sin foco en la app
  useEffect(() => {
    let unlisten;

    getCurrentWindow()
      .listen('tauri://cursor-entered', () => {
        playerRef.current?.play();
      })
      .then((fn) => { unlisten = fn; });

    return () => unlisten?.();
  }, []);

  return (
    <SpritePlayer
      key={`${character}-${state}`}
      ref={playerRef}
      config={config}
      onComplete={onStateEnd}
    />
  );
}
