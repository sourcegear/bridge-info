
mod dotnet;
use dotnet::System;
use dotnet::QuestPDF;

use QuestPDF::Fluent::PageDescriptor;
use QuestPDF::Fluent::TextDescriptor;
use QuestPDF::Fluent::ColumnDescriptor;
use QuestPDF::Infrastructure::Unit;
use QuestPDF::Infrastructure::TextStyle;
use QuestPDF::Helpers::Placeholders;
use QuestPDF::Fluent::PageExtensions::traits::*;
use QuestPDF::Fluent::TextExtensions::traits::*;
use QuestPDF::Fluent::GenerateExtensions::traits::*;
use QuestPDF::Fluent::TextSpanDescriptorExtensions::traits::*;
use QuestPDF::Fluent::PaddingExtensions::traits::*;
use QuestPDF::Fluent::ColumnExtensions::traits::*;
use QuestPDF::Fluent::ImageExtensions::traits::*;
use QuestPDF::Fluent::AlignmentExtensions::traits::*;
use QuestPDF::Fluent::TextStyleExtensions::traits::*;

fn create_pdf(container : Option<QuestPDF::Infrastructure::IDocumentContainer>) -> Result<(), System::Exception> {
    container.unwrap().Page(
        |page : Option<PageDescriptor>|
        {
            let page = page.unwrap();
            page.Size_PageSize(&QuestPDF::Helpers::PageSizes::get_Letter()?)?;
            page.Margin(2.0, Unit::Centimetre())?;
            page.PageColor(&QuestPDF::Helpers::Colors::White())?;
            page.DefaultTextStyle_Func(
                |x : Option<TextStyle>|
                {
                    x.unwrap().FontSize(20.0)
                }
            )?;
            page.Header()?
                .Text_String(Some(&System::String::from("Hello PDF!")))?
                .SemiBold()?.unwrap()
                .FontSize(36.0)?
                ;
            page.Content()?
                .PaddingVertical(1.0, Unit::Centimetre())?
                .Column(
                    |x : Option<ColumnDescriptor>|
                    {
                        let x = x.unwrap();
                        x.Spacing(20.0, Unit::Point())?;
                        x.Item()?.Text_String(Some(&Placeholders::LoremIpsum()?))?;
                        x.Item()?.Image_arru8_ImageScaling(
                            &Placeholders::Image_i32_i32(200, 100)?,
                            QuestPDF::Infrastructure::ImageScaling::FitWidth()
                        )?;
                        Ok(())
                    }
                )?;
            page.Footer()?
                .AlignCenter()?
                .Text_Action(
                    |x : Option<TextDescriptor>|
                    {
                        let x = x.unwrap();
                        x.Span_String(Some(&System::String::from("Page ")))?;
                        x.CurrentPageNumber()?;
                        Ok(())
                    }
                )?;
            Ok(())
        }
    )?;
    Ok(())
}

fn main() -> Result<(), System::Exception> 
{
    QuestPDF::Fluent::Document::Create( create_pdf )?
        .GeneratePdf_String(System::String::from("howdy.pdf"))?
        ;

    return Ok(());
}

