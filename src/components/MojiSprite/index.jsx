import { useState, useCallback } from 'react';
import SpritePlayer from './SpritePlayer';
import { REGISTRY, DEFAULT_CHARACTER, DEFAULT_STATE } from './registry';

// API pública: <MojiSprite character="kael" state="default" onClick={fn} />
// - character: nombre del personaje (default: "kael")
// - state: nombre del estado (default: "default")
// - onClick: callback al hacer clic
// - onStateEnd: callback cuando termina una animación no-loop (regresa a default automáticamente)

export default function MojiSprite({
  character = DEFAULT_CHARACTER,
  state = DEFAULT_STATE,
  onClick,
  onStateEnd,
}) {
  const [internalState, setInternalState] = useState(state);

  const config =
    REGISTRY[character]?.[internalState] ??
    REGISTRY[DEFAULT_CHARACTER][DEFAULT_STATE];

  const handleComplete = useCallback(() => {
    onStateEnd?.();
    setInternalState(DEFAULT_STATE); // Regresa a default al terminar animación no-loop
  }, [onStateEnd]);

  // Sincroniza estado externo → interno
  if (state !== internalState && REGISTRY[character]?.[state]) {
    setInternalState(state);
  }

  return (
    <SpritePlayer
      key={internalState} // Fuerza reinicio al cambiar estado
      config={config}
      onComplete={handleComplete}
      onClick={onClick}
    />
  );
}
