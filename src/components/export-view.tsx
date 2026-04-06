import { addMonths, format, startOfMonth } from "date-fns";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { api } from "@/lib/api";
import { DateNav } from "./date-nav";

export function ExportView() {
  const [date, setDate] = useState(new Date());
  const [output, setOutput] = useState<string>("");
  const [copied, setCopied] = useState(false);
  const [loading, setLoading] = useState(false);

  const generateExport = async () => {
    setLoading(true);
    try {
      const from = format(startOfMonth(date), "yyyy-MM-dd");
      const to = format(addMonths(startOfMonth(date), 1), "yyyy-MM-dd");
      const result = await api.exportSummary(from, to);
      setOutput(result);
    } catch (e) {
      setOutput(`Error generating report: ${e}`);
    } finally {
      setLoading(false);
    }
  };

  const copyToClipboard = async () => {
    await navigator.clipboard.writeText(output);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <ScrollArea className="h-full">
      <div className="flex flex-col gap-3 p-3">
        <div className="flex items-center justify-between">
          <DateNav
            date={date}
            mode="month"
            onDateChange={(d) => {
              setDate(d);
              setOutput("");
            }}
          />
        </div>

        <Button disabled={loading} onClick={generateExport}>
          {loading
            ? "Generating..."
            : `Generate ${format(date, "MMMM yyyy")} Report`}
        </Button>

        {output && (
          <>
            <Card>
              <CardContent className="p-3">
                <pre className="select-text overflow-x-auto whitespace-pre font-mono text-foreground text-xs">
                  {output}
                </pre>
              </CardContent>
            </Card>
            <Button onClick={copyToClipboard} variant="secondary">
              {copied ? "Copied!" : "Copy to Clipboard"}
            </Button>
          </>
        )}
      </div>
    </ScrollArea>
  );
}
