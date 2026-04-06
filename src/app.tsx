import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ExportView } from "@/components/export-view";
import { Settings } from "@/components/settings";
import { StatusBar } from "@/components/status-bar";
import { Summary } from "@/components/summary";
import { Timeline } from "@/components/timeline";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { TooltipProvider } from "@/components/ui/tooltip";
import "./app.css";

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 10_000,
    },
  },
});

function AppContent() {
  return (
    <div className="dark flex h-screen flex-col bg-background text-foreground">
      <StatusBar />

      <Tabs
        className="flex flex-1 flex-col overflow-hidden"
        defaultValue="today"
      >
        <TabsList className="w-full justify-start rounded-none border-b bg-transparent px-2">
          <TabsTrigger value="today">Today</TabsTrigger>
          <TabsTrigger value="summary">Summary</TabsTrigger>
          <TabsTrigger value="export">Export</TabsTrigger>
          <TabsTrigger className="ml-auto" value="settings">
            Settings
          </TabsTrigger>
        </TabsList>

        <TabsContent className="mt-0 flex-1 overflow-hidden" value="today">
          <Timeline />
        </TabsContent>
        <TabsContent className="mt-0 flex-1 overflow-hidden" value="summary">
          <Summary />
        </TabsContent>
        <TabsContent className="mt-0 flex-1 overflow-hidden" value="export">
          <ExportView />
        </TabsContent>
        <TabsContent className="mt-0 flex-1 overflow-hidden" value="settings">
          <Settings />
        </TabsContent>
      </Tabs>
    </div>
  );
}

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <TooltipProvider>
        <AppContent />
      </TooltipProvider>
    </QueryClientProvider>
  );
}

export default App;
