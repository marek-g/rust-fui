# Layout Alignment

There are two attached values that can be added to any control to precisely position control inside dedicated area: `HorizontalAlignment` and `VerticalAlignment`. Both can be set to one of the value:
- `Alignment::Start`
- `Alignment::Center`
- `Alignment::End`
- `Alignment::Stretch`

## Rules for control developers

Each control is responsible for respecting `HorizontalAlignment` and `VerticalAlignment` attached values on its own. Each layout control should pass whole dedicated area to its child (regardless of measured child size), and the child will position itself.  
