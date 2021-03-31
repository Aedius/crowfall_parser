

const fileSelector = document.getElementById('file-selector');
const timeBetween = document.getElementById('time-between');

fileSelector.addEventListener('change', (event) => {

    const fileList = event.target.files;
    for (const file of fileList) {
        readFile(file)
    }

});

var chart_emit_by_kind = null;
var chart_received_by_kind = null;

function readFile(file) {
    const reader = new FileReader();

    reader.addEventListener('load', (event) => {

        let res = window.parse( event.target.result, BigInt(timeBetween.value,10))
        if (res.errors.length> 0){
            alert("cannot parse the following lines : \n" + res.errors.join("\n"))
        }

        render_bar( "chart_received_by_kind", res.dps_stats.received_by_kind);
        render_bar( "chart_emit_by_kind", res.dps_stats.emit_by_kind);
        render_bar( "chart_received_by_enemy", res.dps_stats.received_by_enemy);
        render_bar( "chart_emit_by_enemy", res.dps_stats.emit_by_enemy);

    });
    reader.readAsText(file);
}

var chart_by_id ={};


function render_bar(id, data){

    if (chart_by_id[id] != null){
        chart_by_id[id].destroy();
    }

    let series = []

    for (var key in data) {
        series.push({
          x: key,
          y:  data[key],
        })
    }

    series.sort(function(a, b) {
      return b.y - a.y ;
    })

    var options = {
      chart: {
        type: 'treemap'
      },
      series: [
          {
            data: series
            }
        ]

    }

    let obj = new ApexCharts(document.querySelector("#"+id), options);
    obj.render();

    chart_by_id[id] = obj
}