use seed::{prelude::*, *};

mod leafy;

const OSLO_POLYGON: &'static str = r#"
{
  "type": "Feature",
  "geometry": {
    "type": "Polygon",
    "coordinates": [
      [
        [
          10.489165172838884,
          60.017259872374645
        ],
        [
          10.580764868996987,
          60.0762384207017
        ],
        [
          10.592122568549627,
          60.09394183519897
        ],
        [
          10.572782530207661,
          60.11678480264957
        ],
        [
          10.600720249305056,
          60.13160981872188
        ],
        [
          10.68031961054535,
          60.13353032001292
        ],
        [
          10.73711867703991,
          60.125733600579316
        ],
        [
          10.78802079942288,
          60.06755422118711
        ],
        [
          10.819765048019693,
          60.064296771632726
        ],
        [
          10.811720634337512,
          60.02561911878851
        ],
        [
          10.876109308200913,
          59.98547372050647
        ],
        [
          10.933734244914053,
          59.97416166211912
        ],
        [
          10.951389441905969,
          59.94924298867558
        ],
        [
          10.914816194580183,
          59.91161920924281
        ],
        [
          10.907158498257449,
          59.869893465966655
        ],
        [
          10.933102370207493,
          59.83659145034232
        ],
        [
          10.936527591798225,
          59.831669697457514
        ],
        [
          10.88029688872709,
          59.81138930328435
        ],
        [
          10.770788935602035,
          59.82510863617183
        ],
        [
          10.744019668227386,
          59.83928320264522
        ],
        [
          10.73100663891497,
          59.877178566827084
        ],
        [
          10.658082484659966,
          59.884410483442366
        ],
        [
          10.632783389561938,
          59.915118906971855
        ],
        [
          10.63388386110467,
          59.95342058502221
        ],
        [
          10.610456248652959,
          59.97660952873646
        ],
        [
          10.55585521816055,
          59.99672657430896
        ],
        [
          10.518070354830757,
          59.999291170702094
        ],
        [
          10.489165172838884,
          60.017259872374645
        ]
      ]
    ]
  },
  "properties": {
    "kommunenummer": "0301",
    "objtype": "Kommune",
    "lokalid": "173018",
    "oppdateringsdato": null,
    "datauttaksdato": "20191220110355",
    "versjonid": "4.1",
    "opphav": null,
    "samiskforvaltningsomrade": false,
    "datafangstdato": null,
    "navnerom": "http://skjema.geonorge.no/SOSI/produktspesifikasjon/AdmEnheter/4.1",
    "navn": [
      {
        "rekkefolge": "",
        "sprak": "nor",
        "navn": "Oslo"
      }
    ]
  }
}"#;

type Model = ();

enum Msg {}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    ()
}

fn update(_msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {}

fn view(_model: &Model) -> impl IntoNodes<Msg> {
    let feature_group = leafy::feature_group()
        .with(leafy::geojson(OSLO_POLYGON))
        .with(leafy::marker(59.9147857, 10.7470423).tooltip("Hello Oslo!"))
        .zoom_to_fit();

    div![leafy::map().with(feature_group)]
}

// START

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
