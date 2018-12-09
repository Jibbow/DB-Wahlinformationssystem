import React, { Component } from 'react'
import C3Chart from 'react-c3js';
import bayern_map from '../../assets/Bayern_Landtagswahlkreise_2018.svg';

const data = {
  columns: [
    ['data1', 30],
    ['data2', 70]
  ],
  type: 'donut'
};


export default class LandtagView extends Component {
    render() {
        return (
            <div>
                <div class="container">
                    <img src={bayern_map} className="bayern-map" alt="Karte von Bayern" />
                </div>
                <div class="container-fluid">
                    <C3Chart data={data} />
                </div>
            </div>
        );
    }
}
