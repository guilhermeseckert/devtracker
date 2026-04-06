import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Switch } from "@/components/ui/switch";
import { api } from "@/lib/api";

export function Settings() {
  const queryClient = useQueryClient();

  const { data: autostart } = useQuery({
    queryKey: ["autostart"],
    queryFn: () => api.getAutostart(),
  });

  const toggleAutostart = useMutation({
    mutationFn: (enabled: boolean) => api.setAutostart(enabled),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ["autostart"] }),
  });

  return (
    <ScrollArea className="h-full">
      <div className="flex flex-col gap-3 p-3">
        <h2 className="font-medium text-muted-foreground text-xs uppercase tracking-wider">
          Settings
        </h2>

        <Card>
          <CardHeader className="px-3 pt-3 pb-2">
            <CardTitle className="text-sm">General</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-3 px-3 pb-3">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-sm">Launch at login</p>
                <p className="text-muted-foreground text-xs">
                  Start DevTracker automatically when you log in
                </p>
              </div>
              <Switch
                checked={autostart ?? false}
                onCheckedChange={(checked) => toggleAutostart.mutate(checked)}
              />
            </div>
          </CardContent>
        </Card>

        <Separator />

        <Card>
          <CardHeader className="px-3 pt-3 pb-2">
            <CardTitle className="text-sm">About</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-2 px-3 pb-3">
            <div className="flex justify-between">
              <span className="text-muted-foreground text-xs">Version</span>
              <span className="text-xs">0.1.0</span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground text-xs">Database</span>
              <span className="font-mono text-xs">
                ~/Library/.../tracker.db
              </span>
            </div>
            <div className="flex justify-between">
              <span className="text-muted-foreground text-xs">Tracking</span>
              <span className="text-xs">
                Active app, Git branches, Zoom, Idle
              </span>
            </div>
          </CardContent>
        </Card>
      </div>
    </ScrollArea>
  );
}
