import { useEffect, useRef, useState } from "react";
import { Input } from "@/components/ui/input";

interface Props {
  currentTicket: string | null;
  onCancel: () => void;
  onSave: (ticket: string) => void;
}

export function TagEditor({ currentTicket, onSave, onCancel }: Props) {
  const [value, setValue] = useState(currentTicket ?? "");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    inputRef.current?.focus();
    inputRef.current?.select();
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Enter") {
      // Empty string = clear the tag (revert to auto-detected)
      onSave(value.trim().toUpperCase());
    } else if (e.key === "Escape") {
      onCancel();
    }
  };

  return (
    <Input
      className="h-5 w-24 px-2 text-[10px]"
      onBlur={onCancel}
      onChange={(e) => setValue(e.target.value)}
      onKeyDown={handleKeyDown}
      placeholder="empty = clear"
      ref={inputRef}
      type="text"
      value={value}
    />
  );
}
