import { useRef, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import SpritePlayer from './SpritePlayer';
import { REGISTRY, DEFAULT_CHARACTER, DEFAULT_STATE } from './registry';

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
    const timer = setTimeout(() => playerRef.current?.play(), 100);
    return () => clearTimeout(timer);
  }, [state]);

  // Escucha el evento de hover emitido por el hilo Rust (OS-level, sin focus)
  useEffect(() => {
    let unlisten;
    listen('moji:hover', () => {
      playerRef.current?.play();
    }).then((fn) => { unlisten = fn; });
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
