
mod dotnet;

use dotnet::AsBase;
use dotnet::System;
use dotnet::Avalonia;

use Avalonia::AppBuilderDesktopExtensions::traits::*;

use Avalonia::Controls::Panel_Methods;
use Avalonia::Controls::ContentControl_Methods;
use Avalonia::Collections::AvaloniaList_1_Methods;
use Avalonia::Layout::Layoutable_Methods;

fn main() -> Result<(), System::Exception> 
{
    let lifetime = Avalonia::Controls::ApplicationLifetimes::ClassicDesktopStyleApplicationLifetime::new()?;

    Avalonia::AppBuilder::Configure::<Avalonia::Application>()?
        .UsePlatformDetect()?
        .AfterSetup(
            |b : Option<Avalonia::AppBuilder>| 
            {
                let ft = Avalonia::Themes::Fluent::FluentTheme::new(Option::<System::IServiceProvider>::None)?;
                b.unwrap().get_Instance()?.unwrap().get_Styles()?.Add(&ft)?;
                Ok(())
            }
            )?
        .SetupWithLifetime(&lifetime)?
        ;

    let w = Avalonia::Controls::Window::new()?;
    w.set_Title(System::String::from("Avalonia from Rust"))?;

    let sp = Avalonia::Controls::StackPanel::new()?;
    sp.set_VerticalAlignment(Avalonia::Layout::VerticalAlignment::Center())?;
    sp.set_HorizontalAlignment(Avalonia::Layout::HorizontalAlignment::Center())?;

    let tb = Avalonia::Controls::TextBlock::new()?;
    tb.set_Text(System::String::from("Hello World"))?;
    tb.set_FontSize(32.0)?;
    sp.get_Children()?.Add(&tb.as_base())?;

    let lab = Avalonia::Controls::TextBlock::new()?;
    lab.set_Text(System::String::from("0 clicks"))?;
    lab.set_FontSize(24.0)?;
    sp.get_Children()?.Add(&lab.as_base())?;

    let mut count_clicks = 0;

    let btn = Avalonia::Controls::Button::new()?;
    btn.set_Content(System::String::from("Click Me"))?;
    btn.add_Click(
        |_s, _e|
        {
            count_clicks = count_clicks + 1;
            let v = format!("{} clicks", count_clicks);
            lab.set_Text(System::String::from(v.as_str()))?;
            Ok(())
        }
        )?;
    sp.get_Children()?.Add(&btn.as_base())?;

    w.set_Content(&sp)?;

    lifetime.set_MainWindow(&w)?;

    // TODO this is a terrible way to create an empty array
    let args  = System::Collections::Generic::List_1::<System::String>::new()?.ToArray()?;

    lifetime.Start(&args)?;

    return Ok(());
}

