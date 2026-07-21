import { reactive } from "vue";

export type DialogTone = "default" | "danger";

export interface DialogOptions {
  title: string;
  message: string;
  confirmLabel?: string;
  tone?: DialogTone;
}

export interface PromptOptions extends DialogOptions {
  placeholder?: string;
  value?: string;
  secret?: boolean;
}

type DialogKind = "alert" | "confirm" | "prompt";
type DialogResult = boolean | string | null;

export const dialogState = reactive({
  open: false,
  kind: "alert" as DialogKind,
  title: "",
  message: "",
  confirmLabel: "Confirm",
  tone: "default" as DialogTone,
  value: "",
  placeholder: "",
  secret: false,
});

let pendingResolve: ((value: DialogResult) => void) | null = null;

function openDialog(kind: DialogKind, options: DialogOptions | PromptOptions) {
  if (pendingResolve) pendingResolve(null);

  dialogState.open = true;
  dialogState.kind = kind;
  dialogState.title = options.title;
  dialogState.message = options.message;
  dialogState.confirmLabel = options.confirmLabel ?? "Confirm";
  dialogState.tone = options.tone ?? "default";
  dialogState.value = kind === "prompt" ? (options as PromptOptions).value ?? "" : "";
  dialogState.placeholder =
    kind === "prompt" ? (options as PromptOptions).placeholder ?? "" : "";
  dialogState.secret = kind === "prompt" && Boolean((options as PromptOptions).secret);

  return new Promise<DialogResult>((resolve) => {
    pendingResolve = resolve;
  });
}

function settle(value: DialogResult) {
  const resolve = pendingResolve;
  pendingResolve = null;
  dialogState.open = false;
  resolve?.(value);
}

export const dialogs = {
  async alert(options: DialogOptions): Promise<void> {
    await openDialog("alert", { ...options, confirmLabel: options.confirmLabel ?? "OK" });
  },
  async confirm(options: DialogOptions): Promise<boolean> {
    return (await openDialog("confirm", options)) === true;
  },
  async prompt(options: PromptOptions): Promise<string | null> {
    const result = await openDialog("prompt", options);
    return typeof result === "string" ? result : null;
  },
  accept() {
    settle(dialogState.kind === "prompt" ? dialogState.value : true);
  },
  cancel() {
    settle(dialogState.kind === "alert" ? true : null);
  },
};
