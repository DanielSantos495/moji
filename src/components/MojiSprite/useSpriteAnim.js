import { useEffect, useRef, useState } from 'react';

// Hook que maneja el loop de animación con requestAnimationFrame.
// Devuelve el frame actual (número) para que SpritePlayer lo use.
export function useSpriteAnim({ frames, fps, loop, onComplete }) {
  const [frame, setFrame] = useState(0);
  const rafRef = useRef(null);
  const lastTimeRef = useRef(null);
  const frameRef = useRef(0);
  const interval = 1000 / fps;

  useEffect(() => {
    // Reinicia al cambiar la animación
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
            setFrame(frames - 1);
            onComplete?.();
            return; // Detiene el loop
          }
        }

        setFrame(frameRef.current);
      }

      rafRef.current = requestAnimationFrame(tick);
    }

    rafRef.current = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(rafRef.current);
  }, [frames, fps, loop]); // Se reinicia si cambia la animación

  return frame;
}
