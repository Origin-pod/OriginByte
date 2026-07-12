declare global {
  interface Window {
    __ssrAuth?: {
      unlock: () => void;
      lock: () => void;
      isUnlocked: () => boolean;
    };
  }
}
export {};
