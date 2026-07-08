import { useEffect, useRef } from "react";
import { Send, Square } from "lucide-react";
import { Bot } from "lucide-react";

export function ChatEmptyState() {
  return (
    <div className="flex flex-col items-center justify-center h-[50vh] text-zinc-500 space-y-4">
      <Bot className="w-12 h-12 opacity-50" />
      <p className="text-sm">Start a conversation with the AISDK AI Agent</p>
    </div>
  );
}

interface ChatInputAreaProps {
  input: string;
  handleInputChange: (
    e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => void;
  handleSubmit: (e: React.SubmitEvent<HTMLFormElement>) => void;
  isLoading?: boolean;
  stop?: () => void;
}

export function ChatInputArea({
  input,
  handleInputChange,
  handleSubmit,
  isLoading,
  stop,
}: ChatInputAreaProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  // Auto-resize textarea
  useEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";
      textareaRef.current.style.height = `${Math.min(
        textareaRef.current.scrollHeight,
        200
      )}px`;
    }
  }, [input]);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      // Only submit if input has value and form submission function is available
      if (input.trim() && !isLoading) {
        handleSubmit(e as unknown as React.SubmitEvent<HTMLFormElement>);
      }
    }
  };

  return (
    <div className="bg-zinc-950/80 backdrop-blur-md border-t border-zinc-800/50 p-4 shrink-0">
      <form
        onSubmit={handleSubmit}
        className="max-w-3xl mx-auto relative flex items-end gap-2 bg-zinc-900 border border-zinc-800 rounded-xl overflow-hidden focus-within:ring-1 focus-within:ring-zinc-600 focus-within:border-zinc-700 transition-all shadow-sm"
      >
        <textarea
          ref={textareaRef}
          value={input}
          onChange={handleInputChange}
          onKeyDown={handleKeyDown}
          placeholder="Ask me anything about aisdk.rs"
          className="w-full max-h-50 bg-transparent text-zinc-100 placeholder:text-zinc-500 resize-none outline-none py-4 pl-4 pr-12 overflow-y-auto"
          rows={1}
          disabled={isLoading && !stop}
        />

        <div className="absolute right-2 bottom-2 flex gap-1">
          {isLoading ? (
            <button
              type="button"
              onClick={stop}
              className="p-2 text-zinc-400 hover:text-rose-400 bg-zinc-800/50 hover:bg-zinc-800 rounded-lg transition-colors"
              title="Stop generating"
            >
              <Square className="w-5 h-5 fill-current" />
            </button>
          ) : (
            <button
              type="submit"
              disabled={!input.trim()}
              className="p-2 text-zinc-400 hover:text-zinc-100 bg-zinc-800/50 hover:bg-zinc-800 disabled:opacity-50 disabled:hover:text-zinc-400 disabled:hover:bg-zinc-800/50 rounded-lg transition-colors"
              title="Send message"
            >
              <Send className="w-5 h-5" />
            </button>
          )}
        </div>
      </form>
      <div className="text-center mt-2">
        <span className="text-[10px] text-zinc-600 font-mono tracking-wider">
          aisdk.rs
        </span>
      </div>
    </div>
  );
}
