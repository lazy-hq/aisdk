"use client";

import { useChat } from "@ai-sdk/react";
import {
	DefaultChatTransport,
	isToolUIPart,
	getToolName,
	type UIMessage,
} from "ai";
import { useEffect, useRef, useState } from "react";
import { ChatMessageAvatar } from "./components/ChatMessageAvatar";
import { ChatEmptyState, ChatInputArea } from "./components/ChatInputArea";
import {
	ChatMessageText,
	ToolHeader,
	ToolInput,
	ToolResult,
	ToolError,
} from "./components/AgentTools";

type ToolPart = Extract<UIMessage["parts"][number], { toolCallId: string }>;

export default function App() {
	const { messages, sendMessage, stop, status } = useChat({
		transport: new DefaultChatTransport({
			api: "http://localhost:8080/api/chat",
		}),
	});

	const [input, setInput] = useState("");
	const scrollRef = useRef<HTMLDivElement>(null);
	const isLoading = status === "streaming" || status === "submitted";

	const handleInputChange = (
		e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>,
	) => {
		setInput(e.target.value);
	};

	const handleSubmit = (e: React.SubmitEvent<HTMLFormElement>) => {
		e.preventDefault();
		if (input.trim()) {
			sendMessage({ text: input });
			setInput("");
		}
	};

	useEffect(() => {
		if (scrollRef.current) {
			scrollRef.current.scrollIntoView({ behavior: "smooth" });
		}
	}, [messages]);

	return (
		<div className="flex flex-col h-dvh w-full bg-zinc-950 text-zinc-50 font-sans selection:bg-zinc-800">
			<div className="flex-1 overflow-y-auto px-4 py-8 md:px-8 space-y-8 scroll-smooth">
				<div className="max-w-3xl mx-auto space-y-8">
					{messages.length === 0 && <ChatEmptyState />}

					{messages.map((message) => {
						return (
							<div key={message.id} className="flex flex-row gap-4 group">
								<ChatMessageAvatar role={message.role} />

								<div className="flex flex-col gap-2 min-w-0 flex-1 items-start">
									{message.parts?.map((part, index) => {
										const partKey = `${message.id}-${index}`;

										if (part.type === "text" && part.text) {
											return <ChatMessageText key={partKey} text={part.text} />;
										}

										if (isToolUIPart(part)) {
											const toolPart = part as ToolPart;
											const toolName = getToolName(toolPart);
											const isComplete = toolPart.state === "output-available";
											const hasError = toolPart.state === "output-error";

											return (
												<div
													key={partKey}
													className="flex flex-col gap-2 mt-2 w-full font-mono text-sm"
												>
													<ToolHeader
														toolName={toolName}
														state={toolPart.state}
														isComplete={isComplete}
														hasError={hasError}
													/>

													{toolPart.input != null && (
														<ToolInput input={toolPart.input} />
													)}

													{isComplete && toolPart.output != null && (
														<ToolResult output={toolPart.output} />
													)}

													{hasError && toolPart.errorText && (
														<ToolError errorText={toolPart.errorText} />
													)}
												</div>
											);
										}

										return null;
									})}
								</div>
							</div>
						);
					})}

					<div ref={scrollRef} className="h-px bg-transparent" />
				</div>
			</div>

			<ChatInputArea
				input={input}
				handleInputChange={handleInputChange}
				handleSubmit={handleSubmit}
				isLoading={isLoading}
				stop={stop}
			/>
		</div>
	);
}
