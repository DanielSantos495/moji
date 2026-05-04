import { useRef, useEffect } from 'react';
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

  function handleHover() {
    playerRef.current?.play();
  }

  return (
    <SpritePlayer
      key={`${character}-${state}`}
      ref={playerRef}
      config={config}
      onComplete={onStateEnd}
      onHover={handleHover}
    />
  );
}
