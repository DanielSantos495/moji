import { useRef, useEffect, useState } from 'react';
import SpritePlayer from './SpritePlayer';
import { REGISTRY, DEFAULT_CHARACTER, DEFAULT_STATE, CLICK_STATE } from './registry';

const FOUR_MINUTES = 4 * 60 * 1000;

export default function MojiSprite({ character = DEFAULT_CHARACTER, onStateEnd }) {
  const playerRef = useRef(null);
  const [activeState, setActiveState] = useState(DEFAULT_STATE);
  const isClickPlayingRef = useRef(false);

  const config =
    REGISTRY[character]?.[activeState] ??
    REGISTRY[DEFAULT_CHARACTER][DEFAULT_STATE];

  // Lanza la animación del estado indicado
  function triggerState(nextState) {
    setActiveState(nextState);
    // setTimeout permite que el remount por key tenga tiempo de completarse
    setTimeout(() => playerRef.current?.play(), 100);
  }

  // Reproduce default al iniciar
  useEffect(() => {
    triggerState(DEFAULT_STATE);
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  // Reproduce default cada 4 minutos si no hay un click en curso
  useEffect(() => {
    const id = setInterval(() => {
      if (!isClickPlayingRef.current) triggerState(DEFAULT_STATE);
    }, FOUR_MINUTES);
    return () => clearInterval(id);
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  function handleClick() {
    if (isClickPlayingRef.current) return; // No interrumpir si ya está corriendo
    isClickPlayingRef.current = true;
    triggerState(CLICK_STATE);
  }

  function handleComplete() {
    isClickPlayingRef.current = false;
    setActiveState(DEFAULT_STATE);
    onStateEnd?.();
  }

  return (
    <SpritePlayer
      key={activeState}
      ref={playerRef}
      config={config}
      onComplete={handleComplete}
      onClick={handleClick}
    />
  );
}
