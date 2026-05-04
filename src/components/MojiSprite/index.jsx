import { useRef, useEffect } from 'react';
import SpritePlayer from './SpritePlayer';
import { REGISTRY, DEFAULT_CHARACTER, DEFAULT_STATE } from './registry';

// API pública: <MojiSprite character="kael" state="default" onClick={fn} />
//
// Comportamiento:
//   - Al cargar: dispara la animación automáticamente (una vez)
//   - Al hacer clic: dispara la animación nuevamente
//   - Entre disparos: queda en restFrame (pose estática)
//   - Estados con loop:true (work, sad, etc.) corren de forma continua

export default function MojiSprite({
  character = DEFAULT_CHARACTER,
  state = DEFAULT_STATE,
  onStateEnd,
}) {
  const playerRef = useRef(null);

  const config =
    REGISTRY[character]?.[state] ??
    REGISTRY[DEFAULT_CHARACTER][DEFAULT_STATE];

  // Dispara al cargar el componente
  useEffect(() => {
    playerRef.current?.play();
  }, [state]);

  function handleClick() {
    playerRef.current?.play();
  }

  return (
    <SpritePlayer
      key={`${character}-${state}`}
      ref={playerRef}
      config={config}
      onComplete={onStateEnd}
      onClick={handleClick}
    />
  );
}
