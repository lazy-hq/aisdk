import { Terminal, Bot } from "lucide-react";
import { cn } from "@/lib/utils";

interface ChatMessageAvatarProps {
  role: "user" | "assistant" | "system" | "data";
}

export function ChatMessageAvatar({ role }: ChatMessageAvatarProps) {
  const isUser = role === "user";

  return (
    <div
      className={cn(
        "shrink-0 w-8 h-8 rounded-md flex items-center justify-center border mt-0.5",
        isUser
          ? "bg-zinc-800 border-zinc-700 text-zinc-300 -mt-1"
          : "bg-zinc-900 border-emerald-900/50 text-emerald-500"
      )}
    >
      {isUser ? (
        <Terminal className="w-4 h-4" />
      ) : (
        <Bot className="w-4 h-4" />
      )}
    </div>
  );
}
