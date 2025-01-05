import githubIcon from '../../assets/icons/github.svg';
import npmIcon from '../../assets/icons/npm.svg';
import cratesIcon from '../../assets/icons/crates.svg';
import vercelIcon from '../../assets/icons/vercel.svg';

const linkClass =
  'group flex items-center gap-1.5 text-gray-600/90 transition-all hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200';
const dividerClass = 'h-4 w-px bg-gray-200/50 dark:bg-gray-700/30';
const imageClass =
  'h-4 w-4 transition-colors [filter:invert(0.4)_sepia(0)_saturate(0)] group-hover:[filter:invert(0)_sepia(0)_saturate(1)] dark:[filter:invert(0.7)_sepia(0)_saturate(0)] dark:group-hover:[filter:invert(0.9)_sepia(0)_saturate(0)]';
const textClass = 'text-xs font-medium tracking-wide';

export const Footer = () => {
  return (
    <footer className="flex justify-center gap-3 py-2">
      <a
        href="https://github.com/xnmeet/rico"
        target="_blank"
        rel="noopener noreferrer"
        className={linkClass}>
        <img src={githubIcon} alt="GitHub" className={imageClass} />
        <span className={textClass}>GitHub</span>
      </a>
      <div className={dividerClass} />
      <a
        href="https://www.npmjs.com/package/@rico-core/parser"
        target="_blank"
        rel="noopener noreferrer"
        className={linkClass}>
        <img
          src={npmIcon}
          alt="NPM"
          className="h-3 w-[42px] transition-colors [filter:invert(0.4)_sepia(0)_saturate(0)] group-hover:[filter:invert(0)_sepia(0)_saturate(1)] dark:[filter:invert(0.7)_sepia(0)_saturate(0)] dark:group-hover:[filter:invert(0.9)_sepia(0)_saturate(0)]"
        />
      </a>
      <div className={dividerClass} />
      <a
        href="https://crates.io/crates/rico"
        target="_blank"
        rel="noopener noreferrer"
        className={linkClass}>
        <img src={cratesIcon} alt="Crates.io" className={imageClass} />
        <span className={textClass}>Crates.io</span>
      </a>
      <div className={dividerClass} />
      <a
        href="https://vercel.com"
        target="_blank"
        rel="noopener noreferrer"
        className={linkClass}>
        <img src={vercelIcon} alt="Vercel" className={imageClass} />
        <span className={textClass}>Vercel</span>
      </a>
    </footer>
  );
};
