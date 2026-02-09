/**
 * Settings store using Svelte 5 runes
 * Persists user preferences to localStorage
 */

export type Language = "vi" | "en";

export type Settings = {
  language: Language;
  autoCheckUpdates: boolean;
};

const STORAGE_KEY = "amlich-settings";

const defaultSettings: Settings = {
  language: "vi",
  autoCheckUpdates: false,
};

function loadSettings(): Settings {
  if (typeof localStorage === "undefined") {
    return defaultSettings;
  }

  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      return { ...defaultSettings, ...parsed };
    }
  } catch (e) {
    console.warn("Failed to load settings from localStorage:", e);
  }

  return defaultSettings;
}

function saveSettings(settings: Settings): void {
  if (typeof localStorage === "undefined") {
    return;
  }

  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch (e) {
    console.warn("Failed to save settings to localStorage:", e);
  }
}

// Create reactive settings state
let _settings = $state<Settings>(loadSettings());

export const settings = {
  get value() {
    return _settings;
  },

  get language() {
    return _settings.language;
  },

  set language(lang: Language) {
    _settings.language = lang;
    saveSettings(_settings);
  },

  get autoCheckUpdates() {
    return _settings.autoCheckUpdates;
  },

  set autoCheckUpdates(value: boolean) {
    _settings.autoCheckUpdates = value;
    saveSettings(_settings);
  },

  toggleLanguage() {
    this.language = _settings.language === "vi" ? "en" : "vi";
  },
};
