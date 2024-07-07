# vega view

Display [nushell](https://www.nushell.sh) tables using [Vega Lite](https://vega.github.io/vega-lite/) in a webview.

## Usage

```nushell
use vega.nu
vega view <title> <spec>              # visualize the input with a given title and vega-lite specification
vega bar {flags} <value>              # generate a vega specification for a bar graph
vega series {flags} <value> <time>    # generate a vega specification for a time series plot
vega scatter {flags} <value> <domain> # generate a vega specification for scatter
vega flip                             # exchange the x and y axes of a vega specification
```

For example, to visualize the `b` column of the table in `example.json`:

```nushell
let title = "My Example"
let spec = vega bar b
let data = open example.json
$data | vega view $title $spec
```

This produces a single bar with height representing the sum of the `b` column.  A more interesting bar graph would divide the data up into categories:

```nushell
open example.json | vega view "Stacked Bar Example" (vega bar b --category=t --subcategory=a)
```

With nushell's excellent data handling abilities you can equally easily visualize CSV, SQLite, JSON and other data sources.  

## Visualizations

You can write your own specification or use one of the built in ones below.  

### Bar Graph

A vega-lite specification for a bar graph.

Usage:
```
  > bar {flags} <value> 
```
Flags:
```
  --category <String> - field to discriminate different bars
  --subcategory <String> - field to discriminate stacked bar sections
  --aggregate <String> - how to combine values for a bar or bar section (default: 'sum')
```
Parameters:
```
  value <string>: field name for the bar height
```

Example:

```nushell
open example.json | vega view "Stacked Bar Example" (vega bar b --category=t --subcategory=a)
```

### Time Series Plot

A vega-lite specification for a time series plot.

Usage:
```
  > series {flags} <value> <time> 
```

Flags:
```
  --category <String> - field to discriminate different series
  --area - render as a stacked area plot
```

Parameters:
```
  value <string>: field name for the series values
  time <string>: field for time values
```

Example:
```nushell
open example.json | vega view "Time Series Example" (vega series b t --category a)
```

### Scatter Plot

A vega-lite specification for a scatter plot.

Usage:
```
  > scatter {flags} <value> <domain> 
```

Flags:
```
  --category <String> - field for the category of the point in the plot
```

Parameters:
```
  value <string>: field name for the y coodinate of a point in the  plot
  domain <string>: field for the x coodinate of a point in the plot
```

Example:

```nushell
open example.json | vega view "Scatter Plot Example" (vega scatter b t --category a)
```

### Writing a Specification

The [vega-lite gallery](https://vega.github.io/vega-lite/examples/)  is a good place to start when developing a visualization.  You can adapt one of these specifications by changing the field names to match your data. You must also change the data url to `/data`.  ie:

```nushell
let spec = {
  ...
  data: { url: '/data' }, 
  ...  
}
$my_data | vega view "My Custom Visualization" $spec
```
