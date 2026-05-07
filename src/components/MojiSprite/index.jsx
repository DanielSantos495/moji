import { useRef, useEffect, useState } from 'react';
import SpritePlayer from './SpritePlayer';
import { REGISTRY, DEFAULT_CHARACTER, DEFAULT_STATE, CLICK_STATE } from './registry';

const INTERVAL_MINUTES = 1 * 60 * 1000;

export default function MojiSprite({ character = DEFAULT_CHARACTER, onStateEnd }) {
  const playerRef = useRef(null);
  const [activeState, setActiveState] = useState(DEFAULT_STATE);
  const isClickPlayingRef = useRef(false);

  const hasState = (state) => Boolean(REGISTRY[character]?.[state]);
  const config = REGISTRY[character]?.[activeState];

  // Lanza la animación del estado indicado. Si el personaje no tiene asset
  // para ese estado, la acción queda bloqueada (no se cae a otro personaje).
  function triggerState(nextState) {
    if (!hasState(nextState)) return;
    setActiveState(nextState);
    // setTimeout permite que el remount por key tenga tiempo de completarse
    setTimeout(() => playerRef.current?.play(), 100);
  }

  // Reproduce default al iniciar y cada vez que cambia el personaje.
  useEffect(() => {
    isClickPlayingRef.current = false;
    triggerState(DEFAULT_STATE);
  }, [character]); // eslint-disable-line react-hooks/exhaustive-deps

  // Reproduce default cada N minutos si no hay un click en curso
  useEffect(() => {
    const id = setInterval(() => {
      if (!isClickPlayingRef.current) triggerState(DEFAULT_STATE);
    }, INTERVAL_MINUTES);
    return () => clearInterval(id);
  }, [character]); // eslint-disable-line react-hooks/exhaustive-deps

  function handleClick() {
    if (isClickPlayingRef.current) return;
    if (!hasState(CLICK_STATE)) return; // bloqueado: este personaje no tiene click
    isClickPlayingRef.current = true;
    triggerState(CLICK_STATE);
  }

  function handleComplete() {
    isClickPlayingRef.current = false;
    setActiveState(DEFAULT_STATE);
    onStateEnd?.();
  }

  if (!config) return null;

  return (
    <SpritePlayer
      key={`${character}-${activeState}`}
      ref={playerRef}
      config={config}
      onComplete={handleComplete}
      onClick={handleClick}
    />
  );
}
