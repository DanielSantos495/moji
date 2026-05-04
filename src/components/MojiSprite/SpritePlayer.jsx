import { useRef, useEffect, useImperativeHandle, forwardRef } from 'react';
import { useSpriteAnim } from './useSpriteAnim';
import './SpritePlayer.css';

// forwardRef permite que MojiSprite llame a player.play() desde afuera
const SpritePlayer = forwardRef(function SpritePlayer({ config, onComplete, onHover }, ref) {
  const { src, frames, cols, rows, frameWidth, frameHeight, fps, loop, restFrame } = config;

  const containerRef = useRef(null);
  const playerRef = useRef(null);

  const { frame, play } = useSpriteAnim({ frames, fps, loop, restFrame, onComplete });

  // Expone play() al componente padre
  useImperativeHandle(ref, () => ({ play }), [play]);

  // Calcula posición del frame en el grid
  const col = frame % cols;
  const row = Math.floor(frame / cols);
  const x = col * frameWidth;
  const y = row * frameHeight;

  // Escala el sprite para llenar el contenedor manteniendo proporción
  useEffect(() => {
    const container = containerRef.current;
    const player = playerRef.current;
    if (!container || !player) return;

    const observer = new ResizeObserver(([entry]) => {
      const { width, height } = entry.contentRect;
      const scale = Math.min(width / frameWidth, height / frameHeight);
      player.style.transform = `scale(${scale})`;
    });

    observer.observe(container);
    return () => observer.disconnect();
  }, [frameWidth, frameHeight]);

  return (
    <div ref={containerRef} className="sprite-container" onMouseEnter={onHover}>
      <div
        ref={playerRef}
        className="sprite-player"
        style={{
          width: frameWidth,
          height: frameHeight,
          backgroundImage: `url(${src})`,
          backgroundSize: `${frameWidth * cols}px ${frameHeight * rows}px`,
          backgroundPosition: `-${x}px -${y}px`,
        }}
      />
    </div>
  );
});

export default SpritePlayer;
