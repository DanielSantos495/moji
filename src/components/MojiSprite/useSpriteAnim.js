import { useEffect, useRef, useState, useCallback } from 'react';

// Maneja el loop de animación con requestAnimationFrame.
// Soporta dos modos:
//   - loop: true  → animación continua (estados como work, sad)
//   - loop: false → se dispara una vez y queda en restFrame (default, celebrate, pet)
export function useSpriteAnim({ frames, fps, loop, restFrame = 0, onComplete }) {
  const [frame, setFrame] = useState(restFrame);
  const [playing, setPlaying] = useState(false);
  const rafRef = useRef(null);
  const lastTimeRef = useRef(null);
  const frameRef = useRef(restFrame);
  const interval = 1000 / fps;

  // Inicia la animación desde el frame 0
  const play = useCallback(() => {
    setPlaying(true);
  }, []);

  useEffect(() => {
    if (!playing && !loop) return;

    frameRef.current = 0;
    lastTimeRef.current = null;
    setFrame(0);

    function tick(timestamp) {
      if (!lastTimeRef.current) lastTimeRef.current = timestamp;

      const elapsed = timestamp - lastTimeRef.current;

      if (elapsed >= interval) {
        lastTimeRef.current = timestamp - (elapsed % interval);
        frameRef.current += 1;

        if (frameRef.current >= frames) {
          if (loop) {
            frameRef.current = 0;
          } else {
            // Terminó: queda en restFrame y notifica
            setFrame(restFrame);
            setPlaying(false);
            onComplete?.();
            return;
          }
        }

        setFrame(frameRef.current);
      }

      rafRef.current = requestAnimationFrame(tick);
    }

    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [playing, loop, frames, fps, restFrame]);

  return { frame, play, playing };
}
