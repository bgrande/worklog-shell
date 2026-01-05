#import "../../dist/templates/invoice-modern.typ": *

// Funktionen zum Parsen der JSON-Daten
#let parse-datetime(dt-str) = {
  if dt-str == "" { none }
  else { datetime(dt-str) }
}

#let create-invoice-from-json(invoice, config) = {
  // company details from config
  let biller = (
    name: config.company.name,
    company: config.company.name,
    vat-id: config.company.vat_id,
    tax-id: config.company.tax_id,
    iban: config.company.iban,
    address: (
      street: config.company.address.street,
      postal-code: config.company.address.zip,
      city: config.company.address.city,
      country: config.company.address.country,
    ),
    letter_text: invoice.payment_text,
  )

  // customer details from invoice
  let recipient = (
    name: invoice.customer_name,
    company: invoice.customer_id,
    vat-id: invoice.vat_id,
    address: (
      street: invoice.recipient_address.street,
      postal-code: invoice.recipient_address.postal_code,
      city: invoice.recipient_address.city,
      country: invoice.recipient_address.country,
    )
  )

  // convert to correct format
  let items = invoice.positions.map(pos => (
    number: pos.position_number,
    date: pos.date,
    description: pos.description,
    dur-min: 0,
    quantity: pos.amount,
    price: float(pos.net_price),
  ))

  // render with default template
  show: invoice.with(
    language: "de", // todo should be dynamic as well -> from customer
    invoice-id: invoice.id,
    customer-id: invoice.customer_id,
    issuing-date: invoice.date_created,
    service-date: parse-datetime(invoice.date_execution),
    delivery-date: parse-datetime(invoice.date_delivery),
    biller: biller,
    recipient: recipient,
    items: items,
    currency: invoice.currency,
  )
}

// main rendering function
#let render-invoice(json-path, config-path) = {
  let invoice-data = json(json-path)
  let config-data = json(config-path)

  create-invoice-from-json(invoice-data, config-data)
}