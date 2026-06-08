interface Umami {
  track: (event: string, data?: Record<string, unknown>) => void;
}

declare global {
  interface Window {
    umami?: Umami;
  }
}

/** Send a custom event to Umami, no-op until the analytics script has loaded. */
export function track(event: string, data?: Record<string, unknown>): void {
  window.umami?.track(event, data);
}
