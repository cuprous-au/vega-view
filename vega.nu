export-env { 
    $env.vega_view_bin = ($env.FILE_PWD | path join "target" "release" "vega-view") 
}

export def view [title: string spec: record] {
    to json | ^$env.vega_view_bin --title $title  ($spec | to json)
}

# vega-lite specification for a bar graph
export def bar [
      value: string          # field name for the bar height
      --category: string     # field to discriminate different bars
      --subcategory: string  # field to discriminate stacked bar sections
      --aggregate: string = 'sum' # how to combine values for a bar or bar section
    ] {
    {
      '$schema': 'https://vega.github.io/schema/vega-lite/v5.json',
      data: { url: '/data' }, 
      mark: { 'type': 'bar', tooltip: true } ,
      "width": "container",
      encoding: {
        ...(
          if $category != null {
            {
              x: { field: $category, type: 'nominal' }
            }
          } else {
            {}
          }
        ),
        y: {
          aggregate: $aggregate,
          field: $value,
          type: 'quantitative',
          axis: {
            title: $"($aggregate) of ($value)"
          }
        }
        ...(
          if $subcategory != null {
            {
              "color": {
                "field": $subcategory,
                "type": "nominal",
              }
            }
          } else {
            {}
          }
        )
      },
    }
}

# vega-lite specification for a time series plot
export def series [
      value: string       # field name for the series values
      time: string        # field for time values
      --category: string  # field to discriminate different series
      --area              # render as a stacked area plot
    ] {
    let mark = if $area { "area" } else { "line" }
    {
      '$schema': 'https://vega.github.io/schema/vega-lite/v5.json',
      data: { url: '/data' }, 
      mark: { 
        type: $mark,
        tooltip: true 
      },
      "width": "container",
      encoding: {
        x: { 
          field: $time, 
          type: 'temporal'
        }, 
        y: {
          field: $value,
          type: 'quantitative',
        },
        ...(
          if $category != null {
            {
              "color": {
                "field": $category,
                "type": "nominal",
              }
            }
          } else {
            {}
          }
        )
      }
    }
}

# vega-lite specification for a scatter plot
export def scatter [
  value: string      # field name for the y coodinate of a point in the  plot 
  domain: string     # field for the x coodinate of a point in the plot 
  --category: string # field for the category of the point in the plot
] {
  {
    '$schema': 'https://vega.github.io/schema/vega-lite/v5.json',
    data: { url: '/data' }, 
    mark: { 'type': 'point', tooltip: true } ,
    "width": "container",
    encoding: {
      x: {
        field: $domain,
        type: 'quantitative',
      },
      y: {
        field: $value,
        type: 'quantitative',
      }
      ...(
        if $category != null {
          {
            "color": {
              "field": $category,
              "type": "nominal",
            },
            "shape": {
              "field": $category,
              "type": "nominal",
            }
          }
        } else {
          {}
        }
      )
    },
  }
}

# swap the x and y axis of a vega-lite specification
export def flip [] {
  let spec = $in
  $spec | update encoding.y $spec.encoding.x | update encoding.x $spec.encoding.y
}