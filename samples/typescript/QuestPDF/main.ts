
import { System } from "./generated/ts/internal.ts";
import { QuestPDF } from "./generated/ts/internal.ts";
import { Unit } from "./generated/ts/QuestPDF/Infrastructure/Unit.ts";
import { Placeholders } from "./generated/ts/QuestPDF/Helpers/Placeholders.ts";

function create_pdf(container : QuestPDF.Infrastructure.IDocumentContainer | null)
{
    container!.Page(
        page =>
        {
            page!.Size(QuestPDF.Helpers.PageSizes.Letter);
            page!.Margin(2, Unit.Centimetre);
            page!.PageColor(QuestPDF.Helpers.Colors.White);
            page!.DefaultTextStyle(
                x => x!.FontSize(20)
            );
            page!.Header()
                .Text("Hello PDF!")
                .SemiBold()
                .FontSize(36);
            page!.Content()
                .PaddingVertical(1, Unit.Centimetre)
                .Column(
                    x =>
                    {
                        x!.Spacing(20, Unit.Point);
                        x!.Item().Text(Placeholders.LoremIpsum());
                        x!.Item().Image(
                            Placeholders.Image(200, 100),
                            QuestPDF.Infrastructure.ImageScaling.FitWidth
                        );
                    }
                );
            page!.Footer()
                .AlignCenter()
                .Text(
                    x =>
                    {
                        x!.Span("Page ");
                        x!.CurrentPageNumber();
                    }
                );
        }
    );
}

QuestPDF.Fluent.Document.Create( create_pdf )
    .GeneratePdf("foo.pdf");

