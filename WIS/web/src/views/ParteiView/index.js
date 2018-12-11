import React, { Component } from 'react';
import bayern_map from '../../assets/Bayern_Landtagswahlkreise_2018.svg';


export default class ParteiView extends Component {
    constructor(props) {
    super(props);
    this.state = {
        
    };
  }

    render() {
        return (
            <div class="row">
                <div class="col-xs-6">
                    <img src={bayern_map} className="bayern-map" alt="Karte von Bayern" />
                </div>
                <div class="col-xs-6">
                    
                </div>
            </div>
        );
    }

    componentDidMount() {
        let start = performance.now();
    }
}
