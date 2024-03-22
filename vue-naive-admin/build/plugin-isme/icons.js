/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/04 22:48:11
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import { getIcons } from '..'

const PLUGIN_ICONS_ID = 'isme:icons'
export function pluginIcons() {
  return {
    name: 'isme:icons',
    resolveId(id) {
      if (id === PLUGIN_ICONS_ID) return '\0' + PLUGIN_ICONS_ID
    },
    load(id) {
      if (id === '\0' + PLUGIN_ICONS_ID) {
        return `export default ${JSON.stringify(getIcons())}`
      }
    },
  }
}
