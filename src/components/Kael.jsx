import "./Kael.css";

export default function Kael({ onClick }) {
  return (
    <svg
      className="kael-svg"
      viewBox="0 0 100 110"
      xmlns="http://www.w3.org/2000/svg"
      onClick={onClick}
    >
      {/* Mini dragon wings */}
      <path d="M20,58 Q4,46 8,64 Q14,68 20,62" fill="#E0BBE4" opacity="0.85" />
      <path d="M80,58 Q96,46 92,64 Q86,68 80,62" fill="#E0BBE4" opacity="0.85" />

      {/* Body */}
      <ellipse cx="50" cy="72" rx="26" ry="22" fill="#B5EAD7" />

      {/* Fox ears */}
      <polygon points="28,42 20,20 40,36" fill="#B5EAD7" />
      <polygon points="72,42 80,20 60,36" fill="#B5EAD7" />
      {/* Inner ear */}
      <polygon points="28,40 23,26 37,36" fill="#FFDAC1" opacity="0.7" />
      <polygon points="72,40 77,26 63,36" fill="#FFDAC1" opacity="0.7" />

      {/* Head */}
      <ellipse cx="50" cy="48" rx="22" ry="20" fill="#B5EAD7" />

      {/* Eyes */}
      <ellipse cx="42" cy="46" rx="4" ry="4.5" fill="#3a3a3a" />
      <ellipse cx="58" cy="46" rx="4" ry="4.5" fill="#3a3a3a" />
      {/* Eye shine */}
      <circle cx="43.5" cy="44" r="1.5" fill="white" />
      <circle cx="59.5" cy="44" r="1.5" fill="white" />

      {/* Nose */}
      <ellipse cx="50" cy="53" rx="2.5" ry="1.8" fill="#FFDAC1" />

      {/* Smile */}
      <path
        d="M45,57 Q50,62 55,57"
        stroke="#aaa"
        strokeWidth="1.2"
        fill="none"
        strokeLinecap="round"
      />

      {/* Cheek blush */}
      <ellipse cx="36" cy="53" rx="5" ry="3" fill="#FFDAC1" opacity="0.45" />
      <ellipse cx="64" cy="53" rx="5" ry="3" fill="#FFDAC1" opacity="0.45" />

      {/* Belly spot */}
      <ellipse cx="50" cy="74" rx="14" ry="10" fill="#FFDAC1" opacity="0.4" />

      {/* Dragon tail */}
      <path
        d="M72,84 Q88,78 86,92 Q80,98 72,90"
        fill="#B5EAD7"
        stroke="#a8d8c8"
        strokeWidth="0.5"
      />

      {/* Small dragon scale dots on back */}
      <circle cx="44" cy="63" r="1.5" fill="#9dd4c0" opacity="0.6" />
      <circle cx="50" cy="61" r="1.5" fill="#9dd4c0" opacity="0.6" />
      <circle cx="56" cy="63" r="1.5" fill="#9dd4c0" opacity="0.6" />
    </svg>
  );
}
