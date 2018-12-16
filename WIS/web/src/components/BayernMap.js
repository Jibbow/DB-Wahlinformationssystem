import React, { Component } from 'react';
import { ReactComponent as BayernMapSVG } from '../assets/Bayern_Landtagswahlkreise_2018.svg';

export default class BayernMap extends Component {
  constructor(props) {
    super(props);
    this.state = {};
  }

  render() {
    return (
     <BayernMapSVG/>
    );
  }
}
