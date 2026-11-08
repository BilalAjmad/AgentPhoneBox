import { Composition } from 'remotion';
import { MobileBoxLaunch } from './MobileBoxLaunch';

export const RemotionRoot: React.FC = () => {
  return (
    <>
      <Composition
        id="MobileBoxLaunch"
        component={MobileBoxLaunch}
        durationInFrames={845}
        fps={30}
        width={1920}
        height={1080}
      />
    </>
  );
};