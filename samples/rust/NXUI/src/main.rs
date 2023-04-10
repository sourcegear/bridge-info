
mod dotnet;

use dotnet::AsBase;
use dotnet::System;
use dotnet::Avalonia;
use dotnet::NXUI;

use Avalonia::AppBuilderDesktopExtensions::traits::*;
use Avalonia::ClassicDesktopStyleApplicationLifetimeExtensions::traits::*;
use NXUI::Extensions::AppBuilderExtensions::traits::*;
use NXUI::Extensions::ContentControlExtensions::traits::*;
use NXUI::Extensions::WindowExtensions::traits::*;
use NXUI::Extensions::ButtonExtensions::traits::*;
use NXUI::Extensions::PanelExtensions::traits::*;
use NXUI::Extensions::TextBlockExtensions::traits::*;
use System::Reactive::Linq::Observable::traits::*;

fn build() -> Result<Avalonia::Controls::Window, System::Exception>
{
    let mut count_clicks = 0;
    let btn = Avalonia::Controls::Button::new()?;
    let w = Avalonia::Controls::Window::new()?
        .Title_String(System::String::from("Avalonia from Rust"))?.unwrap()
        .Content_Object(
            Avalonia::Controls::StackPanel::new()?
                .Children_Control(
                    Avalonia::Controls::TextBlock::new()?
                        .Text_String(System::String::from("Hello World"))?.unwrap()
                        .FontSize_f64(32.0)?
                        .unwrap()
                    )?
                    .unwrap()
                .Children_Control(
                    btn
                        .Content_Object(System::String::from("Click Me"))?.unwrap()
                    )?
                    .unwrap()
                .Children_Control(
                    Avalonia::Controls::Label::new()?
                        .Content_IObservable_BindingMode_BindingPriority(
                            btn.ObserveOnClick(Avalonia::Interactivity::RoutingStrategies::Bubble())?.unwrap()
                                .Select_Func_2( 
                                    |_| 
                                    {
                                        count_clicks = count_clicks + 1;
                                        let v = format!("{} clicks", count_clicks);
                                        let x : System::Object = System::String::from(v.as_str()).as_base();
                                        Ok(x)
                                    }
                                    )?,
                            Avalonia::Data::BindingMode::TwoWay(),
                            Avalonia::Data::BindingPriority::LocalValue()
                            )?
                        .unwrap()
                    )?
                    .unwrap()
            )?.unwrap()
        ;
    Ok(w)
}

fn main() -> Result<(), System::Exception> 
{
    let lifetime = Avalonia::Controls::ApplicationLifetimes::ClassicDesktopStyleApplicationLifetime::new()?;

    Avalonia::AppBuilder::Configure::<Avalonia::Application>()?
        .UsePlatformDetect()?
        .UseFluentTheme(Option::<Avalonia::Styling::ThemeVariant>::None)?
        .SetupWithLifetime(&lifetime)?
        ;

    let w = build()?;

    lifetime.set_MainWindow(&w)?;

    // TODO this is a terrible way to create an empty array
    let args  = System::Collections::Generic::List_1::<System::String>::new()?.ToArray()?;

    lifetime.Start(&args)?;

    return Ok(());
}

