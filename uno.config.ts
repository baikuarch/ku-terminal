import {
  defineConfig,
  presetWind3,
  presetIcons,
  presetAttributify,
  transformerDirectives,
  transformerVariantGroup,
} from "unocss";

// Colors map to the CSS variables declared in src/styles/tokens.css so that
// utility classes stay in sync with the One Dark design system. Change a value
// there and both hand-written CSS and Uno utilities update together.
export default defineConfig({
  presets: [
    presetWind3(),
    presetAttributify(),
    presetIcons({
      scale: 1.1,
      extraProperties: {
        display: "inline-block",
        "vertical-align": "middle",
      },
    }),
  ],
  transformers: [transformerDirectives(), transformerVariantGroup()],
  theme: {
    colors: {
      primary: "var(--color-primary)",
      "primary-hover": "var(--color-primary-hover)",
      secondary: "var(--color-secondary)",
      tertiary: "var(--color-tertiary)",
      warning: "var(--color-warning)",
      // surfaces
      root: "var(--bg-root)",
      app: "var(--bg-app)",
      surface: "var(--bg-surface)",
      "surface-2": "var(--bg-surface-2)",
      elevated: "var(--bg-elevated)",
      hover: "var(--bg-hover)",
      active: "var(--bg-active)",
      input: "var(--bg-input)",
      // text
      "txt-primary": "var(--text-primary)",
      "txt-secondary": "var(--text-secondary)",
      "txt-muted": "var(--text-muted)",
      "txt-inverted": "var(--text-inverted)",
      // borders
      "border-subtle": "var(--border-subtle)",
      "border-default": "var(--border-default)",
      "border-strong": "var(--border-strong)",
    },
    borderRadius: {
      sm: "var(--radius-sm)",
      md: "var(--radius-md)",
      lg: "var(--radius-lg)",
      xl: "var(--radius-xl)",
    },
    fontFamily: {
      sans: "var(--font-sans)",
      mono: "var(--font-mono)",
    },
  },
  shortcuts: {
    // Common component patterns, expressible as one class.
    "icon-btn":
      "w-30px h-30px grid place-items-center rounded-md text-txt-secondary hover:bg-hover hover:text-txt-primary",
    "field-input":
      "h-8 px-2 bg-input border border-border-subtle rounded-md text-txt-primary text-13px focus:border-primary outline-none",
    "btn-primary":
      "h-8 px-4 rounded-md text-13px font-medium bg-primary text-txt-inverted hover:bg-primary-hover",
    "btn-ghost":
      "h-8 px-4 rounded-md text-13px font-medium text-txt-secondary border border-border-default hover:bg-hover hover:text-txt-primary",
  },
});
