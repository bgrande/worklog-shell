#let nbh = "‑"

// Truncate a number to 2 decimal places
// and add trailing zeros if necessary
// E.g. 1.234 -> 1.23, 1.2 -> 1.20
#let add-zeros = (num) => {
    // Can't use trunc and fract due to rounding errors
    let frags = str(num).split(".")
    let (intp, decp) = if frags.len() == 2 { frags } else { (num, "00") }
    str(intp) + "." + (str(decp) + "00").slice(0, 2)
  }

// From https://stackoverflow.com/a/57080936/1850340
#let verify-iban = (country, iban) => {
    let iban-regexes = (
        DE: regex(
          "^DE[a-zA-Z0-9]{2}\s?([0-9]{4}\s?){4}([0-9]{2})$"
        ),
        GB: regex(
          "^GB[a-zA-Z0-9]{2}\s?([a-zA-Z]{4}\s?){1}([0-9]{4}\s?){3}([0-9]{2})$"
        ),
      )

    if country == none or not country in iban-regexes {
      true
    }
    else {
      iban.find(iban-regexes.at(country)) != none
    }
}

#let parse-date = (date-str) => {
  let parts = date-str.split("-")
  if parts.len() != 3 {
    panic(
      "Invalid date string: " + date-str + "\n" +
      "Expected format: YYYY-MM-DD"
    )
  }
  datetime(
    year: int(parts.at(0)),
    month: int(parts.at(1)),
    day: int(parts.at(2)),
  )
}

#let TODO = box(
  inset: (x: 0.5em),
  outset: (y: 0.2em),
  radius: 0.2em,
  fill: rgb(255,180,170),
)[
  #text(
    size: 0.8em,
    weight: 600,
    fill: rgb(100,68,64)
  )[TODO]
]

#let horizontalrule = [
  #v(8mm)
  #line(
    start: (20%,0%),
    end: (80%,0%),
    stroke: 0.8pt + gray,
  )
  #v(8mm)
]

#let signature-line = line(length: 5cm, stroke: 0.4pt)

#let endnote(num, contents) = [
  #stack(dir: ltr, spacing: 3pt, super[#num], contents)
]

#let languages = (
    en: (
      id: "en",
      country: "GB",
      recipient: "Recipient",
      biller: "Biller",
      invoice: "Invoice",
      cancellation-invoice: "Cancellation Invoice",
      cancellation-notice: (id, issuing-date) => [
        As agreed, you will receive a credit note
        for the invoice *#id* dated *#issuing-date*.
      ],
      invoice-id: "Invoice ID",
      customer-id: "Customer ID",
      issuing-date: "Issuing Date",
      service-date: "Service Date",
      delivery-date: "Delivery Date",
      vat-id: "VatId",
      items: "Items",
      closing: "Thank you for the good cooperation!",
      number: "№",
      date: "Date",
      description: "Description",
      duration: "Duration",
      quantity: "Quantity",
      price: "Price",
      total-time: "Total working time",
      subtotal: "Subtotal",
      discount-of: "Discount of",
      vat: "VAT of",
      no-vat: "Not Subject to VAT",
      reverse-charge: "Reverse Charge",
      total: "Total",
      due-text: val =>
        [Please transfer the money onto following bank account due to *#val*:],
      owner: "Owner",
      iban: "IBAN",
      pages: "Page",
      banking-details: "Banking Details",
      business-details: "Business Details",
    ),
    de: (
      id: "de",
      country: "DE",
      recipient: "Empfänger",
      biller: "Aussteller",
      invoice: "Rechnung",
      cancellation-invoice: "Stornorechnung",
      cancellation-notice: (id, issuing-date) => [
        Vereinbarungsgemäß erhalten Sie hiermit eine Gutschrift
        zur Rechnung *#id* vom *#issuing-date*.
      ],
      invoice-id: "Rechnungs-Nr",
      customer-id: "Kunden-Nr",
      issuing-date: "Rechnungsdatum",
      service-date: "Leistungsdatum",
      delivery-date: "Lieferdatum",
      vat-id: "Ust-Id",
      items: "Leistungen",
      closing: "Vielen Dank für die gute Zusammenarbeit!",
      number: "Nr",
      date: "Datum",
      description: "Beschreibung",
      duration: "Dauer",
      quantity: "Menge",
      price: "Preis",
      total-time: "Gesamtarbeitszeit",
      subtotal: "Zwischensumme",
      discount-of: "Rabatt von",
      vat: "Umsatzsteuer von",
      no-vat: "Nicht Umsatzsteuerpflichtig",
      reverse-charge: "Steuerschuldnerschaft des\nLeistungsempfängers",
      total: "Gesamt",
      due-text: val =>
        [Bitte überweise den Betrag bis *#val* auf folgendes Konto:],
      owner: "Inhaber",
      iban: "IBAN",
      pages: "Seite",
      banking-details: "Bankverbindungen",
      business-details: "Geschäftsdaten",
    ),
  )

#let invoice(
  language: "en",
  currency: "€",
  country: none,
  title: none,
  banner-image: none,
  invoice-id: none,
  customer-id: none,
  cancellation-id: none,
  issuing-date: none,
  delivery-date: none,
  service-date: none,
  due-date: none,
  biller: (:),
  recipient: (:),
  keywords: (),
  hourly-rate: none,
  styling: (:), // font, font-size, margin (sets defaults below)
  items: (),
  discount: none,
  vat: 0.19,
  data: none,
  override-translation: none,
  doc,
) = {
  // Set styling defaults
  styling.font = styling.at("font", default: "Open Sans")
  styling.font-size = styling.at("font-size", default: 11pt)
  styling.margin = styling.at("margin", default: (
    top: 50mm,
    right: 20mm,
    bottom: 45mm,
    left: 20mm,
  ))

  language = if data != none {
    data.at("language", default: language)
  } else { language }

  // Translations
  let t = if type(language) == str { languages.at(language) }
          else if type(language) == dictionary { language }
          else { panic("Language must be either a string or a dictionary.") }

  // override parts of translation, e.g. change word "Invoice" into "Quote"
  if override-translation != none {
    for k in t.keys() {
      if override-translation.at(k, default: none) != none {
        t.insert(k, override-translation.at(k))
      }
    }
  }

  if data != none {
    language = data.at("language", default: language)
    currency = data.at("currency", default: currency)
    country = data.at("country", default: t.country)
    title = data.at("title", default: title)
    banner-image = data.at("banner-image", default: banner-image)
    invoice-id = data.at("invoice-id", default: invoice-id)
    customer-id = data.at("customer-id", default: customer-id)
    cancellation-id = data.at("cancellation-id", default: cancellation-id)
    issuing-date = data.at("issuing-date", default: issuing-date)
    delivery-date = data.at("delivery-date", default: delivery-date)
    service-date = data.at("service-date", default: service-date)
    due-date = data.at("due-date", default: due-date)
    biller = data.at("biller", default: biller)
    recipient = data.at("recipient", default: recipient)
    keywords = data.at("keywords", default: keywords)
    hourly-rate = data.at("hourly-rate", default: hourly-rate)
    styling = data.at("styling", default: styling)
    items = data.at("items", default: items)
    discount = data.at("discount", default: discount)
    vat = data.at("vat", default: vat)
  }

  // Verify inputs
  assert(
    verify-iban(country, biller.iban),
    message: "Invalid IBAN " + biller.iban + " for country " + country
  )

  let signature = ""
  let issuing-date = if issuing-date != none { issuing-date }
        else { datetime.today().display("[year]-[month]-[day]") }

  let invoice-id-norm = if invoice-id != none {
          if cancellation-id != none { cancellation-id }
          else { invoice-id }
        }
        else {
          TODO
          // TODO: Reactivate after Typst supports hour, minute, and second
          // datetime
          //   .today()
          //   .display("[year]-[month]-[day]t[hour][minute][second]")
        }

  set document(
    title: title,
    keywords: keywords,
    date: parse-date(issuing-date),
  )

  let header-text = if cancellation-id != none { t.cancellation-invoice }
        else { t.invoice }

  context {
      if counter(page).get().first() == 1 {
          set page(header-ascent: 10%)
      } else {
          //styling.margin.top = 60mm
          set page(header-ascent: 30%)
      }
  }

  set page(
    margin: styling.margin,
    numbering: none,
    header-ascent: 10%,
    header: context {
      let page_count = str(counter(page).get().first()) + " / " + str(counter(page).get().last())
      if counter(page).get().first() > 1 [
        #grid(
            columns: (1fr, 1fr),
            rows: (auto),
            gutter: 1em,
            align: (top, right),
            box[ = #header-text ],
            table(
              columns: 2,
              align: (left, right),
              inset: 4pt,
              [*#t.invoice-id:*], [#invoice-id-norm],
              [*#t.issuing-date:*], [#issuing-date],
              [#t.customer-id:], [#customer-id],
              [#t.pages], [#page_count],
              [], [],
              [], [],
              [], [],
              [], [],
            )
        )
      // Offset page top margin for banner image (was -12pt)
      ] else [#banner-image]
    },
    footer: context [
        #grid(
            columns: (1fr, 1fr, 1fr),
            rows: (auto),
            gutter: 2.5em,
            box[
                #{
                  if "company" in biller { [ *#biller.company* \ ] }
                  else { [ *#biller.name* \ ] }
                }
                #v(.2em)
                #biller.address.street \
                #biller.address.postal-code #biller.address.city \
                #{if "country" in biller.address { [#biller.address.country \ ] }}
            ],
            box[
                *#t.banking-details* \
                #v(.2em)
                foo
            ],
            box[
                *#t.business-details* \
                #v(.2em)
                test
            ]
        )
    ]
  )

  set par(justify: true)
  set text(
    lang: t.id,
    font: if styling.font != none { styling.font } else { () },
    size: styling.font-size,
  )
  set table(stroke: none)

 // align(left)[#block[
 //   #text(weight: "bold", size: 1.1em)[
 //       #(if biller.company != none { biller.company })
 //   ]
 // ]]

  let delivery-service-date = if delivery-date != none { delivery-date }
        else if service-date != none { service-date }
        else { TOOD }
  let delivery-service-date-title = if delivery-date != none { t.delivery-date }
        else if service-date != none { t.service-date }
        else { TOOD }
  let r-vat-id = if "vat-id" in recipient { recipient.vat-id }
        else { "" }
  let r-vat-id-title = if "vat-id" in recipient { t.vat-id + ":" }
        else { "" }

  context {
      let page_count = counter(page).final()
      let page-title = if page_count.at(0) > 1 { t.pages }
            else { "" }
      let page-count = if page_count.at(0) > 1 { str(here().page()) + " / "  + str(page_count.at(0)) }
            else { "" }

      box(height: 9em)[
          #set text(size: 8pt)
          // todo: this should be gray text color? or smaller weight?
          #biller.name | #recipient.address.street | #biller.address.postal-code #biller.address.city #{if "country" in biller.address { [| #biller.address.country ] }}
          #v(0.1em)
          #set text(size: 11pt)
          #grid(
            columns: (1.6fr, 1fr),
            rows: (auto),
            gutter: 1em,
            align: (left, right),
            box[
                #{if "company" in recipient { [#recipient.company \ ] }}
                #recipient.name \
                #recipient.address.street \
                #recipient.address.postal-code #recipient.address.city \
                #{if "country" in recipient.address { [#recipient.address.country \ ] }}
            ],
            table(
              columns: 2,
              align: (left, right),
              inset: 4pt,
              [*#t.invoice-id:*], [#invoice-id-norm],
              [*#t.issuing-date:*], [#issuing-date],
              [#t.customer-id:], [#customer-id],
              [#delivery-service-date-title:], [#delivery-service-date],
              [#r-vat-id-title], [#r-vat-id],
              [#page-title], [#page-count],
            )
          )
      ]
  }

  v(2em)

  [= #(header-text)]

  [#text(
        if cancellation-id != none {
          (t.cancellation-notice)(invoice-id, issuing-date)
        } else {
          biller.letter_text
        }
  )]

  v(1em)

  [== #t.items]

  v(1em)

  let getRowTotal = row => {
    if row.at("dur-min", default: 0) == 0 {
      row.price * row.at("quantity", default: 1)
    }
    else {
      calc.round(hourly-rate * (row.dur-min / 60), digits: 2)
    }
  }

  let cancel-neg = if cancellation-id != none { -1 } else { 1 }

  table(
    columns: (auto, auto, 1fr, auto, auto, auto, auto),
    align: (col, row) =>
        if row == 0 {
          (right,left,left,center,center,center,center,).at(col)
        }
        else {
          (right,left,left,right,right,right,right,).at(col)
        },
    inset: 6pt,
    table.header(
      // TODO: Add after https://github.com/typst/typst/issues/3734
      // align: (right,left,left,center,center,center,center,),
      table.hline(stroke: 0.5pt),
      [*#t.number*],
      [*#t.date*],
      [*#t.description*],
      [*#t.duration*\ #text(size: 0.8em)[( min )]],
      [*#t.quantity*],
      [*#t.price*\ #text(size: 0.8em)[( #currency )]],
      [*#t.total*\ #text(size: 0.8em)[( #currency )]],
      table.hline(stroke: 0.5pt),
    ),
    ..items
      .enumerate()
      .map(((index, row)) => {
        let dur-min = row.at("dur-min", default: 0)
        let dur-hour = dur-min / 60

        (
          row.at("number", default: index + 1),
          row.date,
          row.description,
          str(if dur-min == 0 { "" } else { dur-min }),
          str(row.at("quantity", default: if dur-min == 0 { "1" } else { "" })),
          str(add-zeros(cancel-neg *
           row.at("price", default: calc.round(hourly-rate * dur-hour, digits: 2))
          )),
          str(add-zeros(cancel-neg * getRowTotal(row))),
        )
      })
      .flatten()
      .map(str),
    table.hline(stroke: 0.5pt),
  )

  let sub-total = items
        .map(getRowTotal)
        .sum()

  let total-duration = items
        .map(row => int(row.at("dur-min", default: 0)))
        .sum()

  let discount-value = if discount == none { 0 }
    else {
      if (discount.type == "fixed") { discount.value }
      else if discount.type == "proportionate" {
        sub-total * discount.value
      }
      else { panic(["#discount.type" is no valid discount type]) }
    }
  let discount-label = if discount == none { 0 }
    else {
      if (discount.type == "fixed") { str(discount.value) + " " + currency }
      else if discount.type == "proportionate" {
        str(discount.value * 100) + " %"
      }
      else { panic(["#discount.type" is no valid discount type]) }
    }
  let has-reverse-charge = {
        "vat-id" in recipient and "vat-id" in biller and biller.vat-id.slice(0, 2) != recipient.vat-id.slice(0, 2)
      }
  let tax = if has-reverse-charge { 0 } else { sub-total * vat }
  let total = sub-total - discount-value + tax

  let table-entries = (
    if total-duration != 0 {
      ([#t.total-time:], [*#total-duration min*])
    },
    if (discount-value != 0) or (vat != 0) {
      ([#t.subtotal:],
      [#{add-zeros(cancel-neg * sub-total)} #currency])
    },
    if discount-value != 0 {
      (
        [#t.discount-of #discount-label
          #{if discount.reason != "" { "(" + discount.reason + ")" }}],
        [-#add-zeros(cancel-neg * discount-value) #currency]
      )
    },
    if not has-reverse-charge and (vat != 0) {
      ([#t.vat #{vat * 100} %:],
        [#{add-zeros(cancel-neg * tax)} #currency]
      )
    },
    if (vat == 0) {([#t.no-vat], [ ])},
    if (has-reverse-charge) {
      ([#t.vat:], text(0.9em)[#t.reverse-charge])
    },
    (
      [*#t.total*:],
      [*#add-zeros(cancel-neg * total) #currency*]
    ),
  )
  .filter(entry => entry != none)

  let grayish = luma(245)

  align(right,
    table(
      columns: 2,
      fill: (col, row) => // if last row
        if row == table-entries.len() - 1 { grayish }
        else { none },
      stroke: (col, row) => // if last row
        if row == table-entries.len() - 1 { (y: 0.5pt, x: 0pt) }
        else { none },
      ..table-entries
        .flatten(),
    )
  )

  v(1em)

  if cancellation-id == none {
    let due-date = if due-date != none { due-date }
          else {
            (parse-date(issuing-date) + duration(days: 14))
              .display("[year]-[month]-[day]")
          }

    (t.due-text)(due-date)

    v(1em)
    align(center)[
      #table(
        fill: grayish,
        // stroke: 1pt + blue,
        // columns: 2, // TODO: Doesn't work for unknown reason
        columns: (8em, auto),
        inset: (col, row) =>
          if col == 0 {
            if row == 0 { (top: 1.2em, right: 0.6em, bottom: 0.6em) }
            else { (top: 0.6em, right: 0.6em, bottom: 1.2em) }
          }
          else {
            if row == 0 { (top: 1.2em, right: 2em, bottom: 0.6em, left: 0.6em) }
            else { (top: 0.6em, right: 2em, bottom: 1.2em, left: 0.6em) }
          },
        align: (col, row) => (right,left,).at(col),
        table.hline(stroke: 0.5pt),
        [#t.owner:], [*#biller.name*],
        [#t.iban:], [*#biller.iban*],
        table.hline(stroke: 0.5pt),
      )
    ]
    v(1em)

    t.closing
  }
  else {
    v(1em)
    align(center, strong(t.closing))
  }

  doc
}