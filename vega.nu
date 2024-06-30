export-env { 
    $env.vega_view_bin = ($env.FILE_PWD | path join "target" "release" "vega-view") 
}

export def view [title: string spec] {
    to json | ^$env.vega_view_bin --title $title  ($spec | to json)
}

export def bars [x: string, y: string, aggregate: string = 'average' ] {
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