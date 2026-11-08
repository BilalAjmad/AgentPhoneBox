import { AbsoluteFill, Series, Sequence, Img, staticFile, interpolate, spring, useCurrentFrame, useVideoConfig } from 'remotion';

// ====== Design Tokens ======
const C = {
  bg: '#06060A',
  accent: '#00D4FF',
  accent2: '#4A9EFF',
  white: '#F0F4FF',
  muted: '#8899B4',
  surface: '#0D1119',
  phoneBody: '#1A1D26',
  green: '#00FF88',
};

const FONT = 'Inter, system-ui, -apple-system, sans-serif';

// ====== Animated Background ======
const Background: React.FC = () => {
  const frame = useCurrentFrame();
  const drift1x = interpolate(frame, [0, 1200], [0, 40], { extrapolateRight: 'clamp' });
  const drift1y = interpolate(frame, [0, 1200], [0, -30], { extrapolateRight: 'clamp' });
  const drift2x = interpolate(frame, [0, 1200], [-30, 20], { extrapolateRight: 'clamp' });
  const drift2y = interpolate(frame, [0, 1200], [40, -20], { extrapolateRight: 'clamp' });
  const op1 = interpolate(frame, [0, 300, 700, 1200], [0.2, 0.4, 0.3, 0.25], { extrapolateRight: 'clamp' });
  const op2 = interpolate(frame, [0, 400, 900, 1200], [0.1, 0.25, 0.15, 0.1], { extrapolateRight: 'clamp' });

  return (
    <AbsoluteFill style={{ overflow: 'hidden', backgroundColor: C.bg }}>
      <div style={{
        position: 'absolute', inset: 0,
        background: 'radial-gradient(ellipse 80% 60% at 50% 0%, #0D1830 0%, #06060A 45%, #020208 100%)',
      }} />
      <div style={{
        position: 'absolute',
        width: 800, height: 800,
        left: '25%', top: '5%',
        transform: `translate(${drift1x}px, ${drift1y}px)`,
        background: 'radial-gradient(circle, rgba(0,212,255,0.07) 0%, transparent 55%)',
        filter: 'blur(50px)',
        opacity: op1,
      }} />
      <div style={{
        position: 'absolute',
        width: 700, height: 700,
        right: '15%', top: '35%',
        transform: `translate(${drift2x}px, ${drift2y}px)`,
        background: 'radial-gradient(circle, rgba(0,255,136,0.04) 0%, transparent 55%)',
        filter: 'blur(60px)',
        opacity: op2,
      }} />
      <div style={{
        position: 'absolute', inset: 0,
        background: 'radial-gradient(ellipse at center, transparent 40%, rgba(0,0,0,0.5) 100%)',
        pointerEvents: 'none',
      }} />
    </AbsoluteFill>
  );
};

// ====== Phone Frame ======
const PhoneFrame: React.FC<{
  screenshot: string;
  objectPosition?: string;
  scale?: number;
  rotation?: number;
  float?: boolean;
}> = ({ screenshot, objectPosition = '50% 42%', scale = 1, rotation = 0, float = true }) => {
  const frame = useCurrentFrame();
  const floatY = float ? Math.sin(frame * 0.025) * 4 : 0;
  const floatX = float ? Math.sin(frame * 0.015) * 2 : 0;

  return (
    <div style={{
      width: 480 * scale,
      height: 1067 * scale,
      background: 'linear-gradient(165deg, #2C3240 0%, #14161E 60%, #0C0D13 100%)',
      borderRadius: 44 * scale,
      padding: 14 * scale,
      boxShadow: '0 50px 100px rgba(0,0,0,0.65), 0 0 0 1px rgba(255,255,255,0.04), inset 0 0 0 1px rgba(255,255,255,0.03)',
      position: 'relative',
      transform: `translate(${floatX}px, ${floatY}px) rotate(${rotation}deg)`,
      transformOrigin: 'center center',
    }}>
      <div style={{
        width: '100%',
        height: '100%',
        borderRadius: 30 * scale,
        overflow: 'hidden',
        background: '#000',
        position: 'relative',
      }}>
        <Img
          src={staticFile(screenshot)}
          style={{
            width: '100%',
            height: '100%',
            objectFit: 'cover',
            objectPosition: objectPosition,
          }}
        />
        <div style={{
          position: 'absolute',
          inset: 0,
          background: 'linear-gradient(135deg, rgba(255,255,255,0.04) 0%, transparent 40%, rgba(255,255,255,0.02) 100%)',
          pointerEvents: 'none',
        }} />
      </div>

      <div style={{
        position: 'absolute',
        top: 22 * scale,
        left: '50%',
        transform: 'translateX(-50%)',
        width: 120 * scale,
        height: 28 * scale,
        background: '#000',
        borderRadius: 14 * scale,
        zIndex: 2,
      }} />
      <div style={{
        position: 'absolute',
        bottom: 18 * scale,
        left: '50%',
        transform: 'translateX(-50%)',
        width: 120 * scale,
        height: 4 * scale,
        background: 'rgba(255,255,255,0.25)',
        borderRadius: 2 * scale,
        zIndex: 2,
      }} />
      <div style={{ position: 'absolute', left: -4 * scale, top: '35%', width: 4 * scale, height: 60 * scale, background: '#1A1D25', borderRadius: '0 4px 4px 0' }} />
      <div style={{ position: 'absolute', left: -4 * scale, top: '50%', width: 4 * scale, height: 80 * scale, background: '#1A1D25', borderRadius: '0 4px 4px 0' }} />
      <div style={{ position: 'absolute', right: -4 * scale, top: '40%', width: 4 * scale, height: 50 * scale, background: '#1A1D25', borderRadius: '4px 0 0 4px' }} />
    </div>
  );
};

// ====== Scene: Hook / Title ======
const HookScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const orbScale = spring({ frame, fps, config: { damping: 200, mass: 1.2 } });
  const orbOpacity = interpolate(frame, [0, 20], [0, 0.6], { extrapolateRight: 'clamp' });

  const nameOpacity = interpolate(frame, [20, 45], [0, 1], { extrapolateRight: 'clamp' });
  const nameY = spring({ frame: frame - 20, fps, config: { damping: 14, mass: 1.2, stiffness: 80 } });
  const nameScale = interpolate(nameY, [0, 1], [0.9, 1]);

  const tagOpacity = interpolate(frame, [50, 75], [0, 1], { extrapolateRight: 'clamp' });
  const tagY = spring({ frame: frame - 50, fps, config: { damping: 16, stiffness: 60 } });

  const lineScale = interpolate(frame, [60, 90], [0, 1], { extrapolateRight: 'clamp' });

  return (
    <AbsoluteFill style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
      <div style={{
        position: 'absolute',
        width: 500, height: 500,
        transform: `scale(${orbScale})`,
        opacity: orbOpacity,
        background: 'radial-gradient(circle, rgba(0,212,255,0.15) 0%, transparent 60%)',
        filter: 'blur(40px)',
      }} />

      <div style={{
        opacity: nameOpacity,
        transform: `translateY(${(1 - nameY) * 40}px) scale(${nameScale})`,
        fontSize: 150,
        fontWeight: 900,
        color: C.white,
        fontFamily: FONT,
        letterSpacing: -6,
        lineHeight: 0.9,
      }}>
        MobileBox
      </div>

      <div style={{
        opacity: tagOpacity,
        transform: `translateY(${(1 - tagY) * 20}px)`,
        fontSize: 38,
        fontWeight: 300,
        color: C.accent,
        fontFamily: FONT,
        marginTop: 24,
        letterSpacing: 1,
      }}>
        Your AI Agent's Real Android Phone.
      </div>

      <div style={{
        width: lineScale * 200,
        height: 2,
        background: `linear-gradient(90deg, transparent, ${C.accent}, transparent)`,
        marginTop: 30,
        opacity: lineScale,
      }} />
    </AbsoluteFill>
  );
};

// ====== Scene: AI Agent Control ======
const AgentControlScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const lines = [
    { text: 'Your AI agent gets', accent: '100% control.', sub: 'Every tap. Every swipe. Every app.' },
    { text: 'A real Android phone', accent: 'in your laptop.', sub: 'Not an emulator. A real phone.' },
    { text: '20 MCP tools.', accent: 'Zero limits.', sub: 'Full automation. Full power.' },
  ];

  return (
    <AbsoluteFill style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', gap: 30 }}>
      {lines.map((line, i) => {
        const delay = i * 35;
        const opacity = interpolate(frame, [delay, delay + 20], [0, 1], { extrapolateLeft: 'clamp', extrapolateRight: 'clamp' });
        const y = spring({ frame: frame - delay, fps, config: { damping: 16, stiffness: 70 } });
        const translateY = (1 - y) * 30;

        return (
          <div key={i} style={{
            opacity,
            transform: `translateY(${translateY}px)`,
            textAlign: 'center',
          }}>
            <div style={{
              fontSize: 48,
              fontWeight: 700,
              color: C.white,
              fontFamily: FONT,
              letterSpacing: -2,
            }}>
              {line.text}{' '}
              <span style={{ color: C.accent }}>{line.accent}</span>
            </div>
            <div style={{
              opacity: interpolate(frame, [delay + 15, delay + 30], [0, 1], { extrapolateLeft: 'clamp', extrapolateRight: 'clamp' }),
              fontSize: 20,
              fontWeight: 400,
              color: C.muted,
              fontFamily: FONT,
              marginTop: 8,
            }}>
              {line.sub}
            </div>
          </div>
        );
      })}

      <div style={{
        opacity: interpolate(frame, [100, 120], [0, 1], { extrapolateLeft: 'clamp', extrapolateRight: 'clamp' }),
        fontSize: 24,
        fontWeight: 600,
        color: C.green,
        fontFamily: FONT,
        marginTop: 30,
        padding: '12px 32px',
        border: `1px solid ${C.green}44`,
        borderRadius: 8,
        background: `${C.green}11`,
      }}>
        100% Control. 100% Real. 100% Yours.
      </div>
    </AbsoluteFill>
  );
};

// ====== Scene: Phone Showcase ======
const PhoneScene: React.FC<{
  screenshot: string;
  title: string;
  subtitle: string;
  objectPosition?: string;
}> = ({ screenshot, title, subtitle, objectPosition = '50% 42%' }) => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const phoneX = spring({ frame, fps, config: { damping: 18, mass: 1.3, stiffness: 60 } });
  const phoneTranslateX = interpolate(phoneX, [0, 1], [400, 0]);
  const phoneOpacity = interpolate(phoneX, [0, 0.3], [0, 1], { extrapolateRight: 'clamp' });

  const titleOpacity = interpolate(frame, [25, 45], [0, 1], { extrapolateRight: 'clamp' });
  const titleY = spring({ frame: frame - 25, fps, config: { damping: 16, stiffness: 70 } });

  const subOpacity = interpolate(frame, [40, 60], [0, 1], { extrapolateRight: 'clamp' });
  const subY = spring({ frame: frame - 40, fps, config: { damping: 16, stiffness: 60 } });

  const lineScale = interpolate(frame, [50, 70], [0, 1], { extrapolateRight: 'clamp' });

  const glowOpacity = interpolate(frame, [0, 30, 60], [0, 0.4, 0.2], { extrapolateRight: 'clamp' });

  return (
    <AbsoluteFill style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', overflow: 'hidden' }}>
      <div style={{
        position: 'absolute',
        left: '15%',
        top: '50%',
        opacity: phoneOpacity,
        transform: `translate(${phoneTranslateX}px, -50%)`,
      }}>
        <div style={{
          position: 'absolute',
          top: '50%',
          left: '50%',
          transform: 'translate(-50%, -50%)',
          width: 600, height: 600,
          background: `radial-gradient(circle, ${C.accent}22, transparent 60%)`,
          filter: 'blur(60px)',
          opacity: glowOpacity,
          pointerEvents: 'none',
        }} />
        <PhoneFrame screenshot={screenshot} objectPosition={objectPosition} />
      </div>

      <div style={{
        position: 'absolute',
        right: '8%',
        top: '50%',
        transform: 'translateY(-50%)',
        width: '35%',
        textAlign: 'left',
      }}>
        <div style={{
          opacity: titleOpacity,
          transform: `translateY(${(1 - titleY) * 25}px)`,
          fontSize: 64,
          fontWeight: 800,
          color: C.white,
          fontFamily: FONT,
          letterSpacing: -3,
          lineHeight: 1.05,
        }}>
          {title}
        </div>

        <div style={{
          width: lineScale * 120,
          height: 2,
          background: `linear-gradient(90deg, ${C.accent}, transparent)`,
          marginTop: 20,
          opacity: lineScale,
        }} />

        <div style={{
          opacity: subOpacity,
          transform: `translateY(${(1 - subY) * 15}px)`,
          fontSize: 26,
          fontWeight: 400,
          color: C.muted,
          fontFamily: FONT,
          marginTop: 20,
          lineHeight: 1.5,
        }}>
          {subtitle}
        </div>
      </div>
    </AbsoluteFill>
  );
};

// ====== Scene: MCP Tools Showcase ======
const MCPScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const tools = [
    'screenshot', 'tap', 'swipe', 'type_text', 'press_key',
    'device_info', 'battery', 'app_list', 'app_launch', 'app_stop',
    'file_push', 'file_pull', 'shell', 'wifi', 'camera',
    'notification', 'clipboard', 'screen_record', 'volume', 'location',
  ];

  return (
    <AbsoluteFill style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
      <div style={{
        opacity: interpolate(frame, [0, 20], [0, 1], { extrapolateRight: 'clamp' }),
        fontSize: 44,
        fontWeight: 800,
        color: C.white,
        fontFamily: FONT,
        marginBottom: 20,
        letterSpacing: -2,
      }}>
        20 MCP Tools. <span style={{ color: C.accent }}>Full Control.</span>
      </div>

      <div style={{
        opacity: interpolate(frame, [10, 30], [0, 1], { extrapolateRight: 'clamp' }),
        fontSize: 20,
        fontWeight: 400,
        color: C.muted,
        fontFamily: FONT,
        marginBottom: 50,
      }}>
        Your AI agent can do everything a human can. And more.
      </div>

      <div style={{
        display: 'flex',
        flexWrap: 'wrap',
        gap: 12,
        justifyContent: 'center',
        maxWidth: 900,
      }}>
        {tools.map((tool, i) => {
          const delay = i * 4 + 30;
          const opacity = interpolate(frame, [delay, delay + 8], [0, 1], { extrapolateLeft: 'clamp', extrapolateRight: 'clamp' });
          const scale = spring({ frame: frame - delay, fps, config: { damping: 14, stiffness: 120 } });

          return (
            <div key={tool} style={{
              opacity,
              transform: `scale(${scale})`,
              padding: '8px 16px',
              background: `${C.accent}11`,
              border: `1px solid ${C.accent}33`,
              borderRadius: 6,
              fontSize: 14,
              fontWeight: 500,
              color: C.accent,
              fontFamily: 'monospace',
            }}>
              {tool}
            </div>
          );
        })}
      </div>

      <div style={{
        opacity: interpolate(frame, [110, 130], [0, 1], { extrapolateLeft: 'clamp', extrapolateRight: 'clamp' }),
        fontSize: 22,
        fontWeight: 600,
        color: C.green,
        fontFamily: FONT,
        marginTop: 40,
        padding: '12px 32px',
        border: `1px solid ${C.green}44`,
        borderRadius: 8,
        background: `${C.green}11`,
      }}>
        Your AI agent. Your real Android phone.
      </div>
    </AbsoluteFill>
  );
};

// ====== Scene: Stats ======
const StatsScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const stats = [
    { value: '3', label: 'Rust Crates', suffix: '' },
    { value: '20', label: 'MCP Tools', suffix: '' },
    { value: '1', label: 'Click Install', suffix: '' },
    { value: '100', label: 'Local', suffix: '%' },
  ];

  return (
    <AbsoluteFill style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
      <div style={{
        opacity: interpolate(frame, [0, 20], [0, 1], { extrapolateRight: 'clamp' }),
        fontSize: 40,
        fontWeight: 800,
        color: C.white,
        fontFamily: FONT,
        marginBottom: 60,
        letterSpacing: -1,
      }}>
        Built for <span style={{ color: C.accent }}>AI agents.</span>
      </div>

      <div style={{ display: 'flex', gap: 80, alignItems: 'center' }}>
        {stats.map((stat, i) => {
          const delay = i * 15 + 20;
          const opacity = interpolate(frame, [delay, delay + 15], [0, 1], { extrapolateLeft: 'clamp', extrapolateRight: 'clamp' });
          const y = spring({ frame: frame - delay, fps, config: { damping: 14, stiffness: 80 } });
          const scale = interpolate(y, [0, 1], [0.8, 1]);

          return (
            <div key={i} style={{
              opacity,
              transform: `scale(${scale})`,
              textAlign: 'center',
            }}>
              <div style={{
                fontSize: 72,
                fontWeight: 900,
                color: C.accent,
                fontFamily: FONT,
                letterSpacing: -3,
                lineHeight: 1,
              }}>
                {stat.value}{stat.suffix}
              </div>
              <div style={{
                fontSize: 18,
                fontWeight: 500,
                color: C.muted,
                fontFamily: FONT,
                marginTop: 12,
                letterSpacing: 1,
                textTransform: 'uppercase' as const,
              }}>
                {stat.label}
              </div>
            </div>
          );
        })}
      </div>
    </AbsoluteFill>
  );
};

// ====== Scene: CTA ======
const CTAScene: React.FC = () => {
  const frame = useCurrentFrame();
  const { fps } = useVideoConfig();

  const logoOpacity = interpolate(frame, [0, 20], [0, 1], { extrapolateRight: 'clamp' });
  const logoScale = spring({ frame, fps, config: { damping: 16, stiffness: 70 } });

  const tagOpacity = interpolate(frame, [25, 45], [0, 1], { extrapolateRight: 'clamp' });
  const tagY = spring({ frame: frame - 25, fps, config: { damping: 16, stiffness: 60 } });

  const subOpacity = interpolate(frame, [45, 65], [0, 1], { extrapolateRight: 'clamp' });

  const urlOpacity = interpolate(frame, [70, 90], [0, 1], { extrapolateRight: 'clamp' });
  const urlY = spring({ frame: frame - 70, fps, config: { damping: 16, stiffness: 60 } });

  const starOpacity = interpolate(frame, [95, 115], [0, 1], { extrapolateRight: 'clamp' });
  const starScale = spring({ frame: frame - 95, fps, config: { damping: 12, stiffness: 90 } });

  const glowOpacity = interpolate(frame, [0, 30], [0, 0.3], { extrapolateRight: 'clamp' });

  return (
    <AbsoluteFill style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
      <div style={{
        position: 'absolute',
        width: 600, height: 600,
        background: `radial-gradient(circle, ${C.accent}15, transparent 60%)`,
        filter: 'blur(50px)',
        opacity: glowOpacity,
      }} />

      <div style={{
        opacity: logoOpacity,
        transform: `scale(${logoScale})`,
        fontSize: 96,
        fontWeight: 900,
        color: C.white,
        fontFamily: FONT,
        letterSpacing: -4,
      }}>
        MobileBox
      </div>

      <div style={{
        opacity: tagOpacity,
        transform: `translateY(${(1 - tagY) * 20}px)`,
        fontSize: 32,
        fontWeight: 300,
        color: C.accent,
        fontFamily: FONT,
        marginTop: 16,
      }}>
        Your AI Agent's Real Android Phone.
      </div>

      <div style={{
        opacity: subOpacity,
        fontSize: 20,
        fontWeight: 400,
        color: C.muted,
        fontFamily: FONT,
        marginTop: 12,
      }}>
        100% control. 20 MCP tools. Zero cloud.
      </div>

      <div style={{
        opacity: urlOpacity,
        transform: `translateY(${(1 - urlY) * 15}px)`,
        fontSize: 22,
        fontWeight: 400,
        color: C.muted,
        fontFamily: 'monospace',
        marginTop: 50,
      }}>
        github.com/yourusername/MobileBox
      </div>

      <div style={{
        opacity: starOpacity,
        transform: `scale(${starScale})`,
        fontSize: 20,
        fontWeight: 600,
        color: C.white,
        fontFamily: FONT,
        marginTop: 20,
        padding: '12px 32px',
        border: `1px solid ${C.accent}44`,
        borderRadius: 8,
        background: `${C.accent}11`,
      }}>
        ⭐ Star us on GitHub
      </div>
    </AbsoluteFill>
  );
};

// ====== Main Composition ======
export const MobileBoxLaunch: React.FC = () => {
  return (
    <AbsoluteFill style={{ backgroundColor: C.bg }}>
      <Background />
      <Series>
        <Series.Sequence durationInFrames={90}>
          <HookScene />
        </Series.Sequence>
        <Series.Sequence durationInFrames={120}>
          <AgentControlScene />
        </Series.Sequence>
        <Series.Sequence durationInFrames={120}>
          <MCPScene />
        </Series.Sequence>
        <Series.Sequence durationInFrames={105}>
          <PhoneScene
            screenshot="ss_home.png"
            title="Real Android. Real Phone."
            subtitle="Your AI agent sees what a human sees."
            objectPosition="50% 45%"
          />
        </Series.Sequence>
        <Series.Sequence durationInFrames={105}>
          <PhoneScene
            screenshot="ss_playstore.png"
            title="Real Apps. Real Downloads."
            subtitle="Your AI agent installs and runs real apps."
            objectPosition="50% 40%"
          />
        </Series.Sequence>
        <Series.Sequence durationInFrames={90}>
          <PhoneScene
            screenshot="ss_camera.png"
            title="Laptop = Phone Camera."
            subtitle="Your AI agent uses your webcam as the phone camera."
            objectPosition="50% 45%"
          />
        </Series.Sequence>
        <Series.Sequence durationInFrames={90}>
          <PhoneScene
            screenshot="ss_settings.png"
            title="Your Data. Your Folder."
            subtitle="Your AI agent controls everything. Zero cloud."
            objectPosition="50% 45%"
          />
        </Series.Sequence>
        <Series.Sequence durationInFrames={105}>
          <StatsScene />
        </Series.Sequence>
        <Series.Sequence durationInFrames={120}>
          <CTAScene />
        </Series.Sequence>
      </Series>
    </AbsoluteFill>
  );
};
