
import { System } from "./generated/ts/public.ts";
import { Arr } from "./generated/ts/arr.ts";
import { Arr_rt } from "./generated/ts/arr.ts";
import { Avalonia } from "./generated/ts/public.ts";
import { AppBuilder } from "./generated/ts/Avalonia/public.ts";
import { Window } from "./generated/ts/Avalonia/Controls/public.ts";
import { TextBlock } from "./generated/ts/Avalonia/Controls/public.ts";
import { StackPanel } from "./generated/ts/Avalonia/Controls/public.ts";
import { Button } from "./generated/ts/Avalonia/Controls/public.ts";

let lifetime = new Avalonia.Controls.ApplicationLifetimes.ClassicDesktopStyleApplicationLifetime();

AppBuilder.Configure(new Avalonia.Application_rt())
    .UsePlatformDetect()
    .AfterSetup(
        (b : Avalonia.AppBuilder | null) =>
        {
            let ft = new Avalonia.Themes.Fluent.FluentTheme(null);
            b!.Instance!.Styles.Add(ft);
        }
        )
    .SetupWithLifetime(lifetime);

let w = new Window();
w.Title = "Avalonia from TypeScript";

let sp = new StackPanel();
sp.VerticalAlignment = Avalonia.Layout.VerticalAlignment.Center;
sp.HorizontalAlignment = Avalonia.Layout.HorizontalAlignment.Center;

let tb = new TextBlock();
tb.Text = "Hello World";
tb.FontSize = 32.0;
sp.Children.Add(tb);

let lab = new TextBlock();
lab.Text = "0 clicks";
lab.FontSize = 24;
sp.Children.Add(lab);

let count_clicks = 0;

let btn = new Button();
btn.Content = "Click Me";
btn.add_Click(
    (_s, _e) =>
    {
        count_clicks = count_clicks + 1;
        let v = count_clicks + " clicks";
        lab.Text = v;
    }
    );
sp.Children.Add(btn);

w.Content = sp;

lifetime.MainWindow = w;

let args = new Arr(new Arr_rt(new System.String_rt()), 0);
lifetime.Start(args);

