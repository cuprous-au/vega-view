export-env { 
    $env.vega_view_bin = ($env.FILE_PWD | path join "target" "release" "vega-view") 
}

export def expand [title: string spec] {
  let data = $in
  let spec = $spec | insert data { values: $data } | to json

  $"
<!doctype html>
<html>

<head>
  <title>($title)</title>
  <meta charset='utf-8' />

  <script src='https://cdn.jsdelivr.net/npm/vega@5.27.0'></script>
  <script src='https://cdn.jsdelivr.net/npm/vega-lite@5.17.0'></script>
  <script src='https://cdn.jsdelivr.net/npm/vega-embed@6.24.0'></script>
  <style>
    h1 { font-family: sans-serif }
    #vis { width: 100% }
  </style>
</head>

<body>
  <h1>($title)</h1>
  <div id='vis'></div>
  <script>
  vegaEmbed\(
    '#vis', 
    ($spec), 
    { actions: false }
  );
  </script>
</body>

</html>
"
}

export def view [title: string spec] {
    expand $title $spec | ^$env.vega_view_bin view:stdin
}

export def bars [x: string, y: string, aggregate: string = 'average' ] {
    {
      '$schema': 'https://vega.github.io/schema/vega-lite/v5.json',
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