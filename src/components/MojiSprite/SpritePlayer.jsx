import { useRef, useEffect } from 'react';
import { useSpriteAnim } from './useSpriteAnim';
import './SpritePlayer.css';

export default function SpritePlayer({ config, onComplete, onClick }) {
  const { src, frames, cols, rows, frameWidth, frameHeight, fps, loop } = config;

  const containerRef = useRef(null);
  const playerRef = useRef(null);

  const frame = useSpriteAnim({ frames, fps, loop, onComplete });

  // Calcula posición del frame actual en el grid
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
    <div
      ref={containerRef}
      className="sprite-container"
      onClick={onClick}
    >
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
}
