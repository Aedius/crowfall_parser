

const fileSelector = document.getElementById('file-selector');
const timeBetween = document.getElementById('time-between');
const timeMinimum = document.getElementById('minimum-time');

fileSelector.addEventListener('change', (event) => {

    const fileList = event.target.files;
    for (const file of fileList) {
        readFile(file)
    }

});

var chart_emit_by_kind = null;
var chart_received_by_kind = null;
var fights = [];


const fight_list = document.getElementById("fight_list");
fight_list.addEventListener('change', (event)=>{
    console.time("display");
    render_all_timer(event.target.value)
    console.timeEnd("display");
})

function readFile(file) {
    const reader = new FileReader();


    reader.addEventListener('load', (event) => {

        console.time("parse");

        let res = window.parse( event.target.result, BigInt(timeBetween.value,10), BigInt(timeMinimum.value,10) )

        console.timeEnd("parse");
        console.time("display");

        if (res.errors.length> 0){
            alert("cannot parse the following lines : \n" + res.errors.join("\n"))
        }

        fights = res.fights;

        fight_list.innerHTML="";
        for (var i = 0; i < fights.length; i++){
            let st = new Date(fights[i].time.start * 1000);
            let nd = new Date(fights[i].time.end *1000);
            fight_list.options[fight_list.options.length]=  new Option(st.toLocaleTimeString() + " -> " + nd.toLocaleTimeString() + " : "+ fights[i].opponent.join(", "), i)
        }

        if (res.fights[0]){
            render_all_timer(0)
        }
        console.timeEnd("display");

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

    let obj = new ApexCharts(document.querySelector(id), options);
    obj.render();

    chart_by_id[id] = obj
}

function render_all_timer(num){

  let received_damage_series =  [{
       name: 'damage received',
       data: fights[num].dps_stats.received_by_seconds
     }, {
       name: 'damage absorbed',
       data: fights[num].dps_stats.received_by_seconds_absorbed
     }];
    render_timer( "#time_damage_received", received_damage_series, ['#d4526e', '#f9a3a4' ]);

  let received_heal_series =  [{
       name: 'heal received',
       data: fights[num].heal_stats.received_by_seconds
     }, {
       name: 'heal absorbed',
       data: fights[num].heal_stats.received_by_seconds_absorbed
     }];

    render_timer( "#time_heal_received", received_heal_series, ['#33b2df', '#69d2e7' ]);


    let emit_damage_series =  [{
       name: 'dps emit',
       data: fights[num].dps_stats.emit_by_seconds
     }, {
       name: 'dps absorbed',
       data: fights[num].dps_stats.emit_by_seconds_absorbed
     }];
    render_timer( "#time_damage_emit", emit_damage_series,  ['#d4526e', '#f9a3a4' ]);

    let emit_heal_series =  [{
       name: 'heal emit',
       data: fights[num].heal_stats.emit_by_seconds
     }, {
       name: 'heal absorbed',
       data: fights[num].heal_stats.emit_by_seconds_absorbed
     }];

    render_timer( "#time_heal_emit", emit_heal_series, ['#33b2df', '#69d2e7' ]);

    render_bar( "#chart_received_by_kind", fights[num].dps_stats.received_by_kind);
    render_bar( "#chart_emit_by_kind", fights[num].dps_stats.emit_by_kind);
    render_bar( "#chart_received_by_enemy", fights[num].dps_stats.received_by_enemy);
    render_bar( "#chart_emit_by_enemy", fights[num].dps_stats.emit_by_enemy);
    render_bar( "#chart_received_by_ally", fights[num].heal_stats.received_by_ally);
    render_bar( "#chart_emit_by_ally", fights[num].heal_stats.emit_by_ally);
}

function render_timer(id, data, colors){
    if (chart_by_id[id] != null){
        chart_by_id[id].destroy();
    }

    var options = {
      series: data,
      chart: {
      type: 'bar',
      height: 350,
      stacked: true,
      toolbar: {
        show: true
      },
      zoom: {
        enabled: true
      }
    },
    responsive: [{
      options: {
        legend: {
          position: 'bottom',
          offsetX: -10,
          offsetY: 0
        }
      }
    }],
    colors: colors,
    plotOptions: {
      bar: {
        borderRadius: 8,
        horizontal: false,
      },
    },
    legend: {
      position: 'right',
      offsetY: 40
    },
    fill: {
      opacity: 1
    }
   };


    var chart = new ApexCharts(document.querySelector(id), options);
    chart.render();
    chart_by_id[id] = chart
 }