export type SumiMetrics = {
  service?: {
    name?: string;
    version?: string;
    uptime_seconds?: number;
  };
  cache?: {
    hits?: number;
    misses?: number;
    hit_rate_percent?: number;
  };
  requests?: {
    total?: number;
    errors?: number;
    error_rate_percent?: number;
  };
};

export type Service = {
  name: string;
  id?: string;
  status: 'online' | 'offline';
  color: string;
  metrics?: SumiMetrics;
  description?: string;
};

class Store {
  mainServices = $state<Service[]>([
    { name: 'shizuku', status: 'online', color: '48c78e' },
    { name: 'sumi', status: 'online', color: '48c78e' },
    { name: 'blair', status: 'offline', color: 'f14668' }
  ]);

  otherServices = $state<Service[]>([
    {
      name: 'discord status',
      id: 'discord',
      status: 'online',
      color: '48c78e',
      description: 'All Systems Operational'
    },
    {
      name: 'top.gg status',
      id: 'topgg',
      status: 'online',
      color: '48c78e',
      description: 'All services are online'
    }
  ]);

  apiLatency = $state<number>(0);

  constructor() {
    this.fetchStatus();
  }

  async fetchStatus() {
    const startTime = window.performance ? window.performance.now() : Date.now();
    try {
      const res = await fetch('http://localhost:3001/api/status');
      if (res.ok) {
        const data = await res.json();
        this.mainServices = data.main_services;
        this.otherServices = data.other_services;
        this.apiLatency = Math.round((window.performance ? window.performance.now() : Date.now()) - startTime);
        return;
      }
    } catch {
      console.warn('Could not connect to localhost:3001, executing client-side fallback');
    }

    // Run client-side TypeScript fallback in the background
    await this.fetchClientFallback();
    this.apiLatency = Math.round((window.performance ? window.performance.now() : Date.now()) - startTime);
  }

  async fetchClientFallback() {
    // Fetch Discord status directly from Discord's official, CORS-friendly API
    const fetchDiscord = async () => {
      try {
        const res = await fetch('https://discordstatus.com/api/v2/summary.json');
        if (!res.ok) throw new Error('Network response was not ok');
        const data = await res.json();
        const indicator = data.status?.indicator || 'none';
        const description = data.status?.description || 'All Systems Operational';
        const isOnline = indicator === 'none' || indicator === 'minor';

        const idx = this.otherServices.findIndex((s) => s.id === 'discord');
        if (idx !== -1) {
          this.otherServices[idx] = {
            ...this.otherServices[idx],
            status: isOnline ? 'online' : 'offline',
            color: isOnline ? '48c78e' : 'f14668',
            description
          };
        }
      } catch (e) {
        console.warn('Discord fallback status fetch failed:', e);
      }
    };

    // Scrape Top.gg status page HTML through a public CORS proxy
    const fetchTopGG = async () => {
      try {
        const res = await fetch('https://api.allorigins.win/raw?url=https://status.top.gg/');
        if (!res.ok) throw new Error('Network response was not ok');
        const text = await res.text();
        const isOnline = text.includes('All services are online') || text.includes('All systems operational');
        const description = isOnline ? 'All services are online' : 'Some services may be experiencing issues';

        const idx = this.otherServices.findIndex((s) => s.id === 'topgg');
        if (idx !== -1) {
          this.otherServices[idx] = {
            ...this.otherServices[idx],
            status: isOnline ? 'online' : 'offline',
            color: isOnline ? '48c78e' : 'f14668',
            description
          };
        }
      } catch (e) {
        console.warn('Top.gg fallback status fetch failed:', e);
      }
    };

    // Parallel execution ensures that both scraping tasks run concurrently
    // without blocking the main event loop or delaying user interaction.
    await Promise.allSettled([fetchDiscord(), fetchTopGG()]);
  }

  get allServices() {
    return [...this.mainServices, ...this.otherServices];
  }

  get offlineCount() {
    return this.allServices.filter((s) => s.status !== 'online').length;
  }

  get totalCount() {
    return this.allServices.length;
  }

  get overallStatus(): 'success' | 'warning' | 'danger' {
    if (this.offlineCount === 0) return 'success';
    if (this.offlineCount < Math.ceil(this.totalCount / 2)) return 'warning';
    return 'danger';
  }

  get statusMessage(): string {
    if (this.overallStatus === 'success') return 'All services are running smoothly.';
    if (this.overallStatus === 'warning') return 'Some services are experiencing issues.';
    return 'Multiple services are currently offline.';
  }
}

export const store = new Store();
