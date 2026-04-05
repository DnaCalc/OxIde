import { useState } from "react";
import { EmptyState } from "./components/EmptyState";
import { MainEditingState } from "./components/MainEditingState";
import { SemanticRichState } from "./components/SemanticRichState";
import { BuildRunState } from "./components/BuildRunState";
import { CommandPaletteState } from "./components/CommandPaletteState";

type ViewState = "empty" | "editing" | "semantic" | "buildrun" | "palette";

function App() {
  const [currentState, setCurrentState] = useState<ViewState>("empty");

  return (
    <div className="min-h-screen bg-[#0a0e14] text-[#e6e6e8] font-mono antialiased">
      {/* State Switcher */}
      <div className="fixed top-4 right-4 z-50 flex gap-2">
        <button
          onClick={() => setCurrentState("empty")}
          className={`px-3 py-1.5 text-xs transition-colors ${
            currentState === "empty"
              ? "bg-[#39bae6] text-[#0a0e14]"
              : "bg-[#1a1f28] text-[#6c7680] hover:text-[#e6e6e8]"
          }`}
        >
          Empty
        </button>
        <button
          onClick={() => setCurrentState("editing")}
          className={`px-3 py-1.5 text-xs transition-colors ${
            currentState === "editing"
              ? "bg-[#39bae6] text-[#0a0e14]"
              : "bg-[#1a1f28] text-[#6c7680] hover:text-[#e6e6e8]"
          }`}
        >
          Editing
        </button>
        <button
          onClick={() => setCurrentState("semantic")}
          className={`px-3 py-1.5 text-xs transition-colors ${
            currentState === "semantic"
              ? "bg-[#39bae6] text-[#0a0e14]"
              : "bg-[#1a1f28] text-[#6c7680] hover:text-[#e6e6e8]"
          }`}
        >
          Semantic
        </button>
        <button
          onClick={() => setCurrentState("buildrun")}
          className={`px-3 py-1.5 text-xs transition-colors ${
            currentState === "buildrun"
              ? "bg-[#39bae6] text-[#0a0e14]"
              : "bg-[#1a1f28] text-[#6c7680] hover:text-[#e6e6e8]"
          }`}
        >
          Build/Run
        </button>
        <button
          onClick={() => setCurrentState("palette")}
          className={`px-3 py-1.5 text-xs transition-colors ${
            currentState === "palette"
              ? "bg-[#39bae6] text-[#0a0e14]"
              : "bg-[#1a1f28] text-[#6c7680] hover:text-[#e6e6e8]"
          }`}
        >
          Palette
        </button>
      </div>

      {/* Render Current State */}
      {currentState === "empty" && <EmptyState />}
      {currentState === "editing" && <MainEditingState />}
      {currentState === "semantic" && <SemanticRichState />}
      {currentState === "buildrun" && <BuildRunState />}
      {currentState === "palette" && <CommandPaletteState />}
    </div>
  );
}

export default App;
