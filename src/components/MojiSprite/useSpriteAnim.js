import { useRef, useState, useCallback, useEffect } from 'react';

export function useSpriteAnim({ frames, fps, loop, restFrame = 0, onComplete }) {
  const [displayFrame, setDisplayFrame] = useState(restFrame);
  const rafRef = useRef(null);
  const isPlayingRef = useRef(false);
  const interval = 1000 / fps;

  const play = useCallback(() => {
    if (isPlayingRef.current) return;
    isPlayingRef.current = true;

    let currentFrame = 0;
    let lastTime = null;
    setDisplayFrame(0);

    function tick(ts) {
      if (!lastTime) lastTime = ts;

      if (ts - lastTime >= interval) {
        lastTime = ts - ((ts - lastTime) % interval);
        currentFrame++;

        if (currentFrame >= frames) {
          if (loop) {
            currentFrame = 0;
          } else {
            isPlayingRef.current = false;
            setDisplayFrame(restFrame);
            onComplete?.();
            return;
          }
        }

        setDisplayFrame(currentFrame);
      }

      rafRef.current = requestAnimationFrame(tick);
    }

    rafRef.current = requestAnimationFrame(tick);
  }, [frames, fps, loop, restFrame, onComplete, interval]);

  // Limpieza al desmontar
  useEffect(() => {
    return () => {
      cancelAnimationFrame(rafRef.current);
      isPlayingRef.current = false;
    };
  }, []);

  return { frame: displayFrame, play };
}
