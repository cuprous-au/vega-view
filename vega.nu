export-env { 
    $env.vega_view_bin = ($env.FILE_PWD | path join "target" "release" "vega-view") 
}

export def view [title: string spec] {
    to json | ^$env.vega_view_bin --title $title  ($spec | to json)
}

export def bars [x: string = "x", y: string = "y", aggregate: string = 'average' ] {
    {
      '$schema': 'https://vega.github.io/schema/vega-lite/v5.json',
      data: { url: '/data' }, 
      mark: { 'type': 'bar', tooltip: true } ,
      "width": "container",
      encoding: {
        x: { field: $x, type: 'nominal' },
        y: {
          aggregate: $aggregate,
          field: $y,
          type: 'quantitative',
          axis: {
            title: $"($aggregate) of ($y)"
          }
        }
      }
    }
}

export def series [x: string = "x", y: string = "y", t: string = "t", aggregate: string = 'average' ] {
    {
      '$schema': 'https://vega.github.io/schema/vega-lite/v5.json',
      data: { url: '/data' }, 
      mark: { 'type': 'line', tooltip: true } ,
      "width": "container",
      encoding: {
        color: { field: $x, type: 'nominal' },
        x: { field: $t, type: 'temporal'}, 
        y: {
          aggregate: $aggregate,
          field: $y,
          type: 'quantitative',
        }
      }
    }
}

export def flip [] {
  let spec = $in
  $spec | update encoding.y $spec.encoding.x | update encoding.x $spec.encoding.y
}